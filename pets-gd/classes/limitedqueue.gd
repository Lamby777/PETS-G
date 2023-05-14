extends RefCounted
class_name LimitedQueue

var queue = []
var capacity = 0;

func _init(l):
	capacity = l
	super()

# Used for stuff like last n steps for
# movement of playable characters

func get_at(i):
	return queue[i]

# TODO: rewrite in GDExtension when Rust becomes stable
# Apparently pushing/popping from front is a slow operation for large arrays?
# https://docs.godotengine.org/en/stable/classes/class_array.html#class-array-method-push-front
func push_front(v):
	queue.push_front(v)
	
	while len(queue) > capacity:
		queue.pop_back()
