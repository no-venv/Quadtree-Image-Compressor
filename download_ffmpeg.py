import requests;
import platform;
from os import getcwd;
from os import chmod;
from io import BytesIO
from zipfile import ZipFile;

def get_machine():
    machine = platform.machine()
    
    if not machine.find("arm") == -1:
        return machine
    
    return platform.architecture()[0][:-3]

link = "https://ffbinaries.com/api/v1/version/latest";

json_of_ffmpeg_bin = requests.get(link);
json_of_ffmpeg_bin = json_of_ffmpeg_bin.json()["bin"];

plat_sys = platform.system().lower()
plat = (plat_sys == "darwin" and "osx" or plat_sys) + "-" + get_machine()

print("Downloading FFMPEG from platform: ",plat)
ffmpeg = requests.get(json_of_ffmpeg_bin[plat]["ffmpeg"])
ffprobe = requests.get(json_of_ffmpeg_bin[plat]["ffprobe"])

zip1 = BytesIO()
zip2 = BytesIO()

zip1.write(ffmpeg.content)
zip2.write(ffprobe.content)

ZipFile(zip1,"r").extractall(getcwd())
ZipFile(zip2,"r").extractall(getcwd())

if plat_sys=="linux":
    print("setting permission")
    chmod("ffmpeg",0o775)
    chmod("ffprobe",0o775)