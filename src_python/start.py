import time
import os, signal

# ==== USR NAME / PIZZA ULTIMATE PASSWORD ====
print(" Set USR Name: ", end="")
usr=input()
print(" Set Pult password: ", end="")
pult=input()

print("")

# ==== AUTHENTICATION ====
print(" Authenticate")
print(" Enter Username: ", end="")
name=input()
print(" Enter Pult password: ", end="")
ps=input()
print("")

# ==== AUTH CHECK ====
if name == usr and ps == pult:
    print(" \033[32mACCESS GRANTED\033[0m")
    print("")
    time.sleep(1.5)
elif name != usr and ps == pult:
    print(" \033[31mPACKON ERROR CODE: 1\033[0m\nWrong USR Name, PackON will not boot.")
    ppid = os.getppid()
    os.kill(ppid, signal.SIGINT)
elif name == usr and ps != pult:
    print(" \033[31mPACKON ERROR CODE: 2\033[0m\nWrong PizzaUltimate password, PackON will not boot.")
    ppid = os.getppid()
    os.kill(ppid, signal.SIGINT)
else:
    print(" \033[31mPACKON ERROR CODE: 3\033[0m\nWrong Pult password and USR Name, PackON will not boot.")
    ppid = os.getppid()
    os.kill(ppid, signal.SIGINT)

# ==== COMMENTS ====
#import os, signal
#
#ppid = os.getppid()
#os.kill(ppid, signal.SIGINT)
