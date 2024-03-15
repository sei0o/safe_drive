define rr
	r $arg0 --exact --nocapture
end

define ac
	rr test_async_action
end

define can
	rr test_async_cancel
end

# define ign
# 	commands
# 		silent
# 		c
# 	end
# end
# 
# define dpr
# 	# dprintf $arg0, "$arg0 %s", $arg1
# 	b $arg0
# 	commands
# 		silent
# 		printf "$arg0 %s", $arg1
# 		c
# 	end
# end

source dpr.py

dpr server.rs:462 "AsyncGoalReceiver is trying to lock SELECTOR"
dpr server.rs:464 "AsyncGoalReceiver acquired lock for SELECTOR; now sending command to selector"
dpr server.rs:478 "AsyncGoalReceiver returns Poll::Pending; dropping lock for SELECTOR"
dpr server.rs:459 "AsyncGoalReceiver returns Poll::Ready; dropping lock for SELECTOR; goal request received" 


dpr server.rs:707 "AsyncResultReceiver is trying to lock SELECTOR"
dpr server.rs:709 "AsyncResultReceiver has acquired lock of SELECTOR; sending command to it"
dpr server.rs:724 "AsyncResultReceiver returns Poll::Pending; dropping lock for SELECTOR"
dpr server.rs:704 "AsyncResultReceiver returns Poll::Ready; dropping lock for SELECTOR; result request received" 


dpr async_actions.rs:74 "[GOAL] calling recv_goal_request"
dpr async_actions.rs:79 "[GOAL] received goal request; sending goal response"
dpr async_actions.rs:131 "[RESULT] calling recv_result_request"
dpr async_actions.rs:137 "[RESULT] received result request; sending result response"
dpr async_actions.rs:163 "[CLIENT] sent goal request"


dpr server.rs:493 "AsyncGoalReceiver is being dropped; trying to lock SELECTOR"
dpr server.rs:494 "AsyncGoalReceiver acquired lock of SELECTOR"
dpr server.rs:738 "AsyncResultReceiver is being dropped; trying to lock SELECTOR"
dpr server.rs:739 "AsyncResultReceiver acquired lock of SELECTOR"

dpr selector.rs:1438 "[SELECTOR] notify_action_server"
# dpr selector.rs:1127 "[SELECTOR] rcl_action_wait_set_add_action_server"

# wake?
dpr server.rs:717 "AsyncResultReceiver is being waked up"
dpr server.rs:470 "AsyncGoalReceiver is being waked up"

dpr async_selector.rs:162 "calling add_action_server_data"

b selector.rs:763
commands
silent
printf "[*] add_action_server_data: len=%d\n", self.action_servers.length
c
end

b selector.rs:1161 if self.action_servers.length>=2
commands
silent
printf "[?] notify: len=%d\n", self.action_servers.length
c
end

b selector.rs:1487
commands
silent
printf "[*] notify_action_server: new m.len=%d\n", m.length
c
end

b selector.rs:1498
commands
silent
printf "[*] notify_action_client: old m.len=%d\n", m.length
c
end

b selector.rs:1555
commands
silent
printf "[*] notify_action_client: new m.len=%d\n", m.length
c
end
