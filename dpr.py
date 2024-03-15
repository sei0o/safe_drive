import gdb


class PrintMessage(gdb.Command):
    def __init__(self):
        super(PrintMessage, self).__init__("dpr", gdb.COMMAND_USER)

    def invoke(self, arg, from_tty):
        args = gdb.string_to_argv(arg)
        BreakpointHandler(args[0], args[1])

PrintMessage()

class BreakpointHandler(gdb.Breakpoint):
    def __init__(self, location, msg):
        super().__init__(location)
        self.msg = msg
    
    def stop(self):
        frame = gdb.selected_frame()
        sal = frame.find_sal()
        filename = sal.symtab.filename
        line_number = sal.line
        th = gdb.selected_thread().num
        
        print(f"[!] [thread {th}] {filename}:{line_number} {self.msg}")
       
        return False

print("loaded dpr.py")


