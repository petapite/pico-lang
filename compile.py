import os
import platform
import time
import shutil
from res.elevate_windows import elevate
elevate()

print("((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((           *(((((((((((((  (((((((((((\n((((((((((((((((((((((((((((((((((((((((((  (((((((, *(((((((((((((  (((((((((((\n((((((((((((((((((((((((((((((((((((((((((  (((((((((((          /(  (((((((((((\n((((((((((((((((((((((((((((((((((((((((((           *(  ((((((. /(  (((((((((((\n(((((((((((((((((((((((((((((((((((((((((((((((((((, *(  ((((((. /(  (((((((((((\n((((((((((((((((((((((((((((((((((((((((((           *(          /(     ((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((\n((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((")

print("Getting platform data...")
os_system = platform.system()

os_list = ["Linux", "Windows", "Darwin"]

print("Checking if platform " + os_system + " is supported.")
if os_system == os_list[0]:
    print("Platform " + os_system + " not yet supported.")
elif os_system == os_list[1]:
    print("Platform Windows is supported! Starting compilation process")
    print("You need rust and rustup to run this, if you don't have both then expect errors.")
    time.sleep(1)
    print("Checking rustup version...\n[START]")
    os.system("rustup --version")
    print("[END]\nInstalling nightly toolchain... (This might take a while)\n[START]")
    os.system("rustup toolchain install nightly")
    print("[END]\nOverriding project's toolchain from stable -> nightly")
    os.system("rustup override set nightly")
    print("Building...")
    os.system("cargo build")
    print("The build process was successful!")
    input_val = input("Do you want to include some examples into Sol? ")
    input_vfc = input_val[0].lower()
    shutil.rmtree("./target/debug/examples/")
    time.sleep(2)
    if input_vfc == "y":
        print("Including examples...")
        shutil.copytree("./examples/", "./target/debug/examples/", symlinks=False, ignore=None, ignore_dangling_symlinks=False, dirs_exist_ok=False)
        time.sleep(2)
        print("Testing `hello_world.sol`...")
        time.sleep(1)
        os.system('.\\target\\debug\\sol.exe ./target/debug/examples/hello_world.sol')
        time.sleep(2)
    print("This should get you started with Sol!\nYou can change, remove and add features, you get it, and you could even help Sol's repository by making pull requests!\nHappy Coding 😀")
elif os_system == os_list[2]:
    print("Platform " + os_system + " not yet supported.")
input_val = input("Press the enter key to exit.")