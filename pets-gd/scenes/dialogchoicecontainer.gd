extends Container

# https://docs.godotengine.org/en/stable/tutorials/ui/gui_containers.html#creating-custom-containers
# Borrowed, not stolen, from the docs. `&script` :)
func _notification(what):
  if what == NOTIFICATION_SORT_CHILDREN:
    # Must re-sort the children
    for c in get_children():
      # Fit to own size
      fit_child_in_rect(c, Rect2(Vector2(), size))

func set_some_setting():
  # Some setting changed, ask for children re-sort.
  queue_sort()
