//!
//! This module contains pretty much everything on
//! the GDExtension side that runs during battles.
//!

use godot::classes::node::ProcessMode;
use godot::classes::object::ConnectFlags;
use godot::classes::{
    AnimatedSprite2D, AnimationPlayer, AudioStreamPlayer, Control, InputEvent,
    PanelContainer, ProgressBar, RichTextLabel, Texture2D, TextureRect, Timer,
};
use godot::prelude::*;

use crate::common::*;
use crate::consts::battle::*;

mod affinities;
mod autoload;
mod midi;
mod player;
mod rhythm;
#[allow(unused)]
pub mod skills;
mod stat_translation;

pub use affinities::Affinities;
pub use autoload::BattleInterface;
use player::BattleIcon;
use rhythm::BattleMusic;

#[derive(PartialEq)]
enum MenuSection {
    Main,
    _Item,
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

    #[init(val = OnReady::manual())]
    battlers: OnReady<Battlers>,
    current_party_member: usize,

    #[init(node = "BattleMusic")]
    music: OnReady<Gd<BattleMusic>>,

    #[init(node = "%BattleChoices/ChoiceAgent")]
    choices: OnReady<Gd<ChoiceAgent>>,

    /// The choice agent for the right panel
    other_choices: Option<Gd<ChoiceAgent>>,

    #[init(node = "AnimationPlayer")]
    animator: OnReady<Gd<AnimationPlayer>>,

    #[init(node = "%EnemySprite")]
    enemy_sprite: OnReady<Gd<AnimatedSprite2D>>,

    #[init(node = "%BattleIcon")]
    icon: OnReady<Gd<BattleIcon>>,

    /// Metronome-like thingy
    #[init(node = "ClickSFX")]
    clicksfx: OnReady<Gd<AudioStreamPlayer>>,

    /// Text saying if you hit, flop, or miss a note
    #[init(node = "%ClickStatus")]
    click_status_txt: OnReady<Gd<RichTextLabel>>,

    /// Something like the "rolling HP bar" feature from EarthBound
    /// or Sans's KR from Undertale. Basically just a number that your
    /// HP bar is going towards. The real HP value will be set ahead
    /// of time, but the bar will slowly move towards it, and you
    /// won't die until both your HP and the bar show zero.
    #[init(val = OnReady::manual())]
    karma_timer: OnReady<Gd<Timer>>,

    #[init(node = "Menu/DualMenu")]
    dualmenu: OnReady<Gd<Control>>,

    #[init(node = "Menu/DualMenu/AnimationPlayer")]
    dualmenu_animator: OnReady<Gd<AnimationPlayer>>,
}

#[godot_api]
impl BattleEngine {
    pub fn take_damage(&mut self, damage: i32) {
        {
            let battler = self.current_battler();
            let mut battler = battler.borrow_mut();
            battler.take_damage(damage.try_into().unwrap());
        }

        self.update_mana_bar();
    }

    fn update_mana_bar(&mut self) {
        let battler = self.current_battler();
        let battler = battler.borrow();

        let mana = battler.battle_stats.mana;

        let mut mana_bar =
            self.base().get_node_as::<ProgressBar>("%InfoBars/ManaBar");
        mana_bar.set("bar_value", &mana.unwrap_or(0).to_variant());

        let max_mana = battler.inherent_stats.max_mana;
        mana_bar.set_max(max_mana.unwrap_or(1).into());
    }

    #[func]
    fn on_karma(&mut self) {
        let battler = self.current_battler();
        let battler = battler.borrow();

        let hp = battler.battle_stats.hp as f64;
        let mut hp_bar =
            self.base().get_node_as::<ProgressBar>("%InfoBars/HPBar");

        // go up instantly, but go down over time
        let hp_bar_value = hp_bar.get("bar_value").to::<f64>();
        let hp_bar_value = if hp >= hp_bar_value {
            hp
        } else {
            hp_bar_value - KARMA_STEP
        };

        // update hp bar
        hp_bar.set("bar_value", &hp_bar_value.to_variant());

        let max_hp = battler.inherent_stats.max_hp;
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

    fn current_battler(&self) -> Rc<RefCell<Battler>> {
        self.battlers.good_guys[self.current_party_member].clone()
    }

    /// slowly fade out the black rectangle over the battle scene
    #[func]
    pub fn animate_in(&mut self) {
        self.animator.set_assigned_animation("fade_in");
        self.animator.play();
    }

    fn open_dualmenu(&mut self) {
        self.dualmenu_animator
            .play_animation_forwards("dualmenu_open", true);

        self.menu_section = Some(MenuSection::Main);

        // enable the choice list
        let mut choices = self.choices.bind_mut();
        choices.enable();
        choices.focus_nth(0);
    }

    fn close_dualmenu(&mut self) {
        self.dualmenu_animator
            .play_animation_forwards("dualmenu_open", false);

        self.menu_section = None;

        // disable the choice list
        self.choices.bind_mut().disable();
        // self.other_choices.as_mut().map(|v| v.bind_mut().disable());
        self.clear_right_panel();
    }

    fn clear_right_panel(&mut self) {
        self.right_panel_destination
            .clone()
            .expect("no right panel node exported")
            .get_children()
            .iter_shared()
            .for_each(|mut v| v.queue_free());
    }

    fn toggle_dualmenu(&mut self) {
        use BattleState::*;

        match self.state {
            // close menu if open
            _ if self.menu_section.is_some() => self.close_dualmenu(),

            // no menu while running away or in intro
            Intro | Attack { running: true } => (),

            // open menu if closed
            // (exhaustive match in case we add more states later)
            Attack { running: false } => self.open_dualmenu(),
        }
    }

    pub fn swap_party_member(&mut self, new_index: usize) {
        let si = si();

        self.current_party_member = new_index;
        let pchar = &si.bind().save.party[new_index];

        // let pchar = &pcb().bind_mut().party_pchars()[new_index];
        godot_print!("Swapped to party member `{}`", pchar);

        // set battle icon sprite
        self.icon.bind_mut().set_icon(pchar);

        // set battle portrait texture
        let mut portrait =
            self.base().get_node_as::<TextureRect>("%PortraitTexture");

        let path = format!("res://assets/textures/portraits/{pchar}.png");
        let texture = load::<Texture2D>(&path);
        portrait.set_texture(&texture);
    }

    #[func]
    pub fn cast_skill(&mut self, skill_id: StringName) {
        godot_print!("Casting skill: {}", skill_id);
        let skill = unwrap_fmt!(
            REGISTRIES.skills.get(&skill_id),
            "skill not found: {skill_id}",
        );

        skill.cast(
            self.current_battler().clone(),
            self.battlers.bad_guys[0].clone(),
            self.battlers.good_guys.clone(),
            self.battlers.bad_guys.clone(),
        );
    }

    #[func]
    pub fn describe_skill(&self, skill_id: StringName) -> String {
        let skill = unwrap_fmt!(
            REGISTRIES.skills.get(&skill_id),
            "skill not found: {skill_id}",
        );

        skill.description()
    }

    fn open_skills_menu(&mut self) {
        self.menu_section = Some(MenuSection::Skill);

        // clear right panel children
        // TODO: animate them out first
        let mut cont = self
            .right_panel_destination
            .clone()
            .expect("no right panel node exported");

        cont.get_children()
            .iter_shared()
            .for_each(|mut v| v.queue_free());

        let mut panel = self
            .skills_menu_scene
            .clone()
            .expect("no skills menu scene exported")
            .instantiate_as::<PanelContainer>();

        panel.set("battle_engine", &self.base().to_variant());
        cont.add_child(&panel);

        let mut agent: Gd<ChoiceAgent> = panel.get("choice_agent").to();
        agent.bind_mut().enable();

        self.other_choices = Some(agent);
        self.choices.bind_mut().disable();

        // animate slide the new menu up
        cont.get_node_as::<AnimationPlayer>("../AnimationPlayer")
            .play_ex()
            .name("margin_slide_up")
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
                // TODO: implement running mechanic described earlier
                pcb().bind_mut().battling.clear();

                // TODO: don't change scenes, just remove the battle
                // stuff since it's all overlayed on top of the world
                change_scene!("world");
            }

            _ => unreachable!(),
        }
    }

    // ------------------------------------------------------------

    #[func]
    fn on_note_hit(&mut self) {
        self.click_status_txt.set_text("Hit!");
        self.clicksfx.play();
    }

    #[func]
    fn on_note_flop(&mut self) {
        self.click_status_txt.set_text("Flop!");
    }

    #[func]
    fn on_note_miss(&mut self) {
        self.click_status_txt.set_text("Miss!")
    }

    // ------------------------------------------------------------

    #[func]
    pub fn intro_over(&mut self) {
        // change state from intro to attack
        self.state = BattleState::Attack { running: false };
        self.music.bind_mut().play_battle_music();

        // start attacking
        // let enemy_data = pcb().bind().battling[0].clone();
        // let enemy_id = &enemy_data.borrow().id; // TODO: COMMENTED TO COMPILE
        let enemy_id = "A_NONNY_MOUSE";

        self.base()
            .get_node_as::<Node>(&format!("Tactics/{enemy_id}"))
            .set_process_mode(ProcessMode::ALWAYS);
    }
}

#[godot_api]
impl INode2D for BattleEngine {
    fn ready(&mut self) {
        self.choices.bind_mut().disable();
        self.battlers.init(pcb().bind().new_battlers());
        self.update_mana_bar();
        self.clear_right_panel();

        {
            let mut timer = Timer::new_alloc();
            self.base_mut().add_child(&timer);
            timer.set_wait_time(KARMA_INTERVAL);
            timer.set_one_shot(false);
            timer.start();

            let callable = self.base().callable("on_karma");
            timer.connect("timeout", &callable);

            self.karma_timer.init(timer.upcast());
        }

        {
            // intro countdown timer setup
            let mut timer = Timer::new_alloc();
            self.base_mut().add_child(&timer);
            timer.set_wait_time(INTRO_COUNTDOWN_SEC);
            timer.start();
            let callable = self.base().callable("intro_over");
            timer
                .connect_ex("timeout", &callable)
                .flags(ConnectFlags::ONE_SHOT.ord() as u32)
                .done();
        }

        connect! {
            self.choices, "selection_confirmed" =>
            self.base(), "on_choice_picked";

            self.music, "note_hit" =>
            self.base(), "on_note_hit";

            self.music, "note_flop" =>
            self.base(), "on_note_flop";

            self.music, "note_miss" =>
            self.base(), "on_note_miss";
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("menu") {
            self.toggle_dualmenu();
        }
    }
}
