extends ColorRect

var program_time = 0;

func _process(delta):
  program_time += delta
  self.material.set_shader_parameter("current_time", program_time)
  
func reset_shader_timer():
  self.material.set_shader_parameter("start_time", program_time)
