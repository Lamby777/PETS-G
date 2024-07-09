//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::engine::node::ProcessMode;
use godot::engine::object::ConnectFlags;
use godot::engine::{
    AnimatedSprite2D, AnimationPlayer, Control, InputEvent, ProgressBar,
    Texture2D, TextureRect, Timer, VBoxContainer,
};
use godot::prelude::*;
use skills::Skill;

use crate::consts::battle::*;
use crate::prelude::*;

mod affinities;
mod midi;
mod player;
mod rhythm;
#[allow(unused)]
pub mod skills;
mod stat_translation;

pub use affinities::{Affinities, AffinityPower};
use player::BattleIcon;
use rhythm::BattleMusic;

#[derive(Debug)]
enum AttackFlopReason {
    /// The beat was not clicked
    Skipped,

    /// The player clicked outside of a beat window
    PoorTiming,
}

#[derive(PartialEq)]
enum MenuSection {
    Main,
    Item,
    Skill,
}

#[derive(Default, PartialEq)]
enum BattleState {
    /// Few seconds countdown before the music starts
    #[default]
    Intro,

    /// Dodging attacks while clicking to the beat
    Attack {
        /// Running away from battle?
        ///
        /// While running away, you won't be able to fight back, but once
        /// the countdown is over, you'll escape the fight.
        running: bool,
    },
    // /// Has the menu open
    // Menu(MenuSection),
}

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct BattleEngine {
    base: Base<Node2D>,

    state: BattleState,
    menu_section: Option<MenuSection>,

    #[export]
    skills_menu_scene: Option<Gd<PackedScene>>,

    #[export]
    right_panel_destination: Option<Gd<Control>>,

    #[init(default = OnReady::manual())]
    battlers: OnReady<Battlers>,
    current_party_member: usize,

    #[init(default = onready_node(&base, "BattleMusic"))]
    music: OnReady<Gd<BattleMusic>>,

    #[init(default = onready_node(&base, "%BattleChoices/ChoiceAgent"))]
    choices: OnReady<Gd<ChoiceAgent>>,

    #[init(default = onready_node(&base, "AnimationPlayer"))]
    animator: OnReady<Gd<AnimationPlayer>>,

    #[init(default = onready_node(&base, "%EnemySprite"))]
    enemy_sprite: OnReady<Gd<AnimatedSprite2D>>,

    #[init(default = onready_node(&base, "%BattleIcon"))]
    icon: OnReady<Gd<BattleIcon>>,

    /// Something like the "rolling HP bar" feature from EarthBound
    /// or Sans's KR from Undertale. Basically just a number that your
    /// HP bar is going towards. The real HP value will be set ahead
    /// of time, but the bar will slowly move towards it, and you
    /// won't die until both your HP and the bar show zero.
    #[init(default = OnReady::manual())]
    karma_timer: OnReady<Gd<Timer>>,
}

#[godot_api]
impl BattleEngine {
    pub fn take_damage(&mut self, damage: i32) {
        self.current_battler_mut()
            .take_damage(damage.try_into().unwrap());

        self.update_mana_bar();
    }

    fn update_mana_bar(&mut self) {
        let battler = self.current_battler();
        let mana = battler.mana();

        let mut mana_bar =
            self.base().get_node_as::<ProgressBar>("%InfoBars/ManaBar");
        mana_bar.set("bar_value".into(), mana.unwrap_or(0).to_variant());

        let max_mana = battler.inherent_stats().max_mana;
        mana_bar.set_max(max_mana.unwrap_or(1).into());
    }

    #[func]
    fn on_karma(&mut self) {
        let battler = self.current_battler();
        let hp = battler.hp() as f64;
        let mut hp_bar =
            self.base().get_node_as::<ProgressBar>("%InfoBars/HPBar");

        // go up instantly, but go down over time
        let hp_bar_value = hp_bar.get("bar_value".into()).to::<f64>();
        let hp_bar_value = if hp >= hp_bar_value {
            hp
        } else {
            hp_bar_value - KARMA_STEP
        };

        // update hp bar
        hp_bar.set("bar_value".into(), hp_bar_value.to_variant());

        let max_hp = battler.inherent_stats().max_hp;
        hp_bar.set_max(max_hp.into());

        // if both the hp bar and the actual hp are zero, die
        if hp <= 0.0 && hp_bar_value <= 0.0 {
            self.on_death();
        }
    }

    #[func]
    fn on_death(&mut self) {
        // cancel battle timers
        self.karma_timer.stop();

        // TODO
        godot_print!("You died!");
    }

    fn current_battler(&self) -> &Box<dyn Battler> {
        &self.battlers.good_guys[self.current_party_member]
    }

    fn current_battler_mut(&mut self) -> &mut Box<dyn Battler> {
        &mut self.battlers.good_guys[self.current_party_member]
    }

    /// slowly fade out the black rectangle over the battle scene
    #[func]
    pub fn animate_in(&mut self) {
        self.animator.set_assigned_animation("fade_in".into());
        self.animator.play();
    }

    fn open_dualmenu(&mut self) {
        self.dualmenu_animator()
            .play_animation_forwards("dualmenu_open", true);

        self.menu_section = Some(MenuSection::Main);

        // enable the choice list
        let mut choices = self.choices.bind_mut();
        choices.enable();
        choices.focus_nth(0);
    }

    fn close_dualmenu(&mut self) {
        self.dualmenu_animator()
            .play_animation_forwards("dualmenu_open", false);

        self.menu_section = None;

        // disable the choice list
        self.choices.bind_mut().disable();
    }

    fn toggle_dualmenu(&mut self) {
        use BattleState::*;

        match self.state {
            // close menu if open
            _ if self.menu_section.is_some() => self.close_dualmenu(),

            // no menu while running away or in intro
            Intro | Attack { running: true } => return,

            // open menu if closed
            // (exhaustive match in case we add more states later)
            Attack { running: false } => self.open_dualmenu(),
        }
    }

    pub fn dualmenu(&self) -> Gd<Control> {
        self.base().get_node_as::<Control>("Menu/DualMenu")
    }

    pub fn dualmenu_animator(&self) -> Gd<AnimationPlayer> {
        self.dualmenu()
            .get_node_as::<AnimationPlayer>("AnimationPlayer")
    }

    pub fn swap_party_member(&mut self, new_index: usize) {
        self.current_party_member = new_index;
        let pchar = self.battlers.good_guys[new_index].id();
        let pchar = PChar::from_godot(pchar.into());
        godot_print!("Swapped to party member `{}`", pchar);

        // set battle icon sprite
        self.icon.bind_mut().set_icon(pchar);

        // set battle portrait texture
        let mut portrait =
            self.base().get_node_as::<TextureRect>("%PortraitTexture");

        let path = format!("res://assets/textures/portraits/{}.png", pchar);
        let texture = load::<Texture2D>(path);
        portrait.set_texture(texture);
    }

    #[func]
    pub fn run_skill(&mut self, skill: Box<dyn Skill>) {
        //
    }

    fn open_skills_menu(&mut self) {
        self.menu_section = Some(MenuSection::Skill);

        // clear right panel children
        // TODO animate them out first
        let mut rpanel = self
            .right_panel_destination
            .clone()
            .expect("no right panel node exported");

        rpanel
            .get_children()
            .iter_shared()
            .for_each(|mut v| v.queue_free());

        let mut scene = self
            .skills_menu_scene
            .clone()
            .expect("no skills menu scene exported")
            .instantiate_as::<VBoxContainer>();

        scene.set("battle_engine".into(), self.base().to_variant());
        rpanel.add_child(scene.upcast());

        // animate slide the new menu up
        rpanel
            .get_node_as::<AnimationPlayer>("../AnimationPlayer")
            .play_ex()
            .name("margin_slide_up".into())
            .done();
    }

    #[func]
    pub fn on_choice_picked(&mut self, choice: Gd<Control>) {
        match choice.get_name().to_string().as_str() {
            "Skills" => self.open_skills_menu(),

            "Items" => todo!(),

            "Swap" => {
                let mut next = self.current_party_member + 1;
                if next >= BATTLE_PARTY_SIZE {
                    next = 0;
                }

                self.swap_party_member(next);
            }

            "Run" => {
                // TODO implement running mechanic described earlier
                pcb().bind_mut().battling.clear();

                // TODO don't change scenes, just remove the battle
                // stuff since it's all overlayed on top of the world
                change_scene!("world");
            }

            _ => unreachable!(),
        }
    }

    // ------------------------------------------------------------

    #[func]
    pub fn intro_over(&mut self) {
        // change state from intro to attack
        self.state = BattleState::Attack { running: false };
        self.music.bind_mut().play_battle_music();

        // start attacking
        let enemy_data = pcb().bind().battling[0].clone();
        let enemy_id = enemy_data.borrow().id();

        self.base()
            .get_node_as::<Node>(format!("Tactics/{}", enemy_id))
            .set_process_mode(ProcessMode::ALWAYS);
    }
}

#[godot_api]
impl INode2D for BattleEngine {
    fn ready(&mut self) {
        self.choices.bind_mut().disable();
        self.battlers.init(pcb().bind().new_battlers());
        self.update_mana_bar();

        {
            let mut timer = Timer::new_alloc();
            self.base_mut().add_child(timer.clone().upcast());
            timer.set_wait_time(KARMA_INTERVAL);
            timer.set_one_shot(false);
            timer.start();

            let callable = self.base().callable("on_karma");
            timer.connect("timeout".into(), callable);

            self.karma_timer.init(timer.upcast());
        }

        {
            // intro countdown timer setup
            let mut timer = Timer::new_alloc();
            self.base_mut().add_child(timer.clone().upcast());
            timer.set_wait_time(INTRO_COUNTDOWN_SEC);
            timer.start();
            let callable = self.base().callable("intro_over");
            timer
                .connect_ex("timeout".into(), callable)
                .flags(ConnectFlags::ONE_SHOT.ord() as u32)
                .done();
        }

        connect! {
            self.choices, "selection_confirmed" =>
            self.base(), "on_choice_picked";
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("menu".into()) {
            self.toggle_dualmenu();
        }
    }

    fn process(&mut self, _delta: f64) {
        //
    }
}
