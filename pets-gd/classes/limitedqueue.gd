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

func push_front(v):
	queue.push_front(v)
	
	while len(queue) > capacity:
		queue.pop_back()
