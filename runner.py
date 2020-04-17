import os
import stat
import subprocess
import gzip
from base64 import b64decode
from tempfile import NamedTemporaryFile

if __name__ == "__main__":
    with NamedTemporaryFile(prefix="arc2020", delete=False) as tmp:
        try:
            tmp.write(gzip.decompress(b64decode(OUTFILE)))
            file_name = tmp.name
        except:
            os.remove(tmp.name)
            raise

    try:
        st = os.stat(file_name)
        os.chmod(file_name, st.st_mode | stat.S_IEXEC)
        subprocess.run([file_name], check=True)
    finally:
        os.remove(file_name)
