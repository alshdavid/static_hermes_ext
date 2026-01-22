// @ts-nocheck

const _readStdin = $SHBuiltin.extern_c(
  { include: "libshremes_stdin.h" },
  function readStdin(): c_ptr {
    throw 0;
  },
);

const _getByte = $SHBuiltin.extern_c(
  { include: "libshremes_stdin.h" },
  function getByte(ptr: c_ptr, index: c_int): c_int {
    throw 0;
  },
);

const _freeString = $SHBuiltin.extern_c(
  { include: "libshremes_stdin.h" },
  function freeString(ptr: c_ptr): void {
    throw 0;
  },
);

function readAllFromStdin(): string {
  const ptr = _readStdin();
  if (!ptr) return "";

  let result = "";
  let i = 0;

  while (true) {
    const charCode = _getByte(ptr, i);
    if (charCode === 0) break; // Stop at null terminator
    result += String.fromCharCode(charCode);
    i++;
  }

  _freeString(ptr);
  return result;
}

const Stdin = {
  readUntilClosed(): string {
    return readAllFromStdin()
  }
}

const Stdout = {
  writeLine(data: string): void {
    // @ts-expect-error
    print(data)
  }
}
