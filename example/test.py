import os
import subprocess

for example in os.listdir():
    if os.path.isdir(example):
        print(f"--- {example} ---")
        os.chdir(example)

        try:
            subprocess.run(["cargo", "clean"], check=True)
            subprocess.run(["cargo", "build", "--release"], check=True)
            subprocess.run(["cargo", "clean"])
            print(f"{example}: success")

        except:
            print(f"{example}: fail")

        finally:
            os.chdir("..")
