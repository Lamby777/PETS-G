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

func get_len() -> int:
	return len(queue)

func get_last():
	return queue[len(queue)-1]
	
func get_first_or(deft):
	if len(queue) == 0:	return deft
	else:				return queue[0]

func get_or_last(i):
	return queue[
		min(i, len(queue)-1)
	]
