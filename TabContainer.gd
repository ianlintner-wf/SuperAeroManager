extends TabContainer

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_TabContainer_ready():
	hide()
	tab_align = TabContainer.ALIGN_LEFT
	set_tab_title(0, 'CEO')
	


func _on_TabContainer_tab_changed(tab):
	var tc = get_tab_control(tab)
	tc.show()
