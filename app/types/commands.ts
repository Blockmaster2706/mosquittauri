enum commands {
	get_servers = "get_servers",
	add_server = "add_server",
	edit_server = "edit_server",
	delete_server = "delete_server",
	select_server = "select_server",
	get_topics = "get_topics",
	add_topic = "add_topic",
	edit_topic = "edit_topic",
	delete_topic = "delete_topic",
	set_topic_enabled = "set_topic_enabled",
	is_listen_all_topics = "is_listen_all_topics",
	set_listen_all_topics = "set_listen_all_topics",
	mqtt_connect = "mqtt_connect",
}
export default commands;
