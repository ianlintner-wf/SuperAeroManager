extends MenuButton

# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var popup
var tabpanel
var on = false
# Called when the node enters the scene tree for the first time.
func _ready():
	popup = get_popup()
	popup.add_item("Test Item")
	tabpanel = get_parent().get_node("TabContainer")
	popup.add_child(tabpanel)
	on = false
# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_AirlineButton_toggeled():
	if (on):
		popup.hide()
		tabpanel.hide()
	else:
		tabpanel.show()
		popup.show()
	
