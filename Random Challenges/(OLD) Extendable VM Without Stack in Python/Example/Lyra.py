from dill import HIGHEST_PROTOCOL, loads as ploads, dumps as pdumps, settings as dsettings
from binascii import hexlify, unhexlify
from inspect import signature
from json import loads, dumps


class Lyra:

    type_map = {
        "int": "I",
        "str": "S",
        "bool": "L",
        "double": "N",
    }

    class Jumpdest:

        def __init__(self, dest):
            self.dest = dest

    class Halt:
        halt = True

    def ptr(stack):
        return stack[-1]

    def pop(stack):
        return stack[1:-1]

    def jumpi(_, condition, dest):
        if condition:
            return Lyra.Jumpdest(dest)

    def jump(_, dest):
        return Lyra.Jumpdest(dest)

    def jumpdest(_, dest_id):
        return Lyra.Jumpdest(dest_id)

    def _return(_):
        return Lyra.Halt

    def __init__(self):

        self.opcode_map = {
            "ptr": hex(0),
            "pop": hex(1),
            "jumpi": hex(2),
            "jump": hex(3),
            "jumpdest": hex(4),
            "return": hex(5)
        }

        self.execution_map = {
            hex(0): {
                "func": Lyra.ptr,
                "args": 0
            },
            hex(1): {
                "func": Lyra.pop,
                "args": 0
            },
            hex(2): {
                "func": Lyra.jumpi,
                "args": 2
            },
            hex(3): {
                "func": Lyra.jump,
                "args": 1
            },
            hex(4): {
                "func": Lyra.jumpdest,
                "args": 1
            },
            hex(5): {
                "func": Lyra._return,
                "args": 0
            }
        }

        self.li = len(self.opcode_map)

    def extend(self, funcname: str, func: any):
        self.li += 1
        li = hex(self.li)
        self.opcode_map[funcname] = li
        self.execution_map[li] = {"func": func,
                                  "args": len(signature(func).parameters) - 1}

    def compile(self, file_in: str, file_out: str):

        if file_out.split(".")[-1] != "lyrica":
            raise Exception("Output must end with extension .lyrica")

        opcode_map = self.opcode_map
        opcode_keys = opcode_map.keys()
        type_map = Lyra.type_map

        with open(file_in, "r") as f:
            lines = list(line.strip() for line in f.readlines())

        code = ""
        for line in lines:
            if line.startswith("#") or line == "":
                continue
            # if " " in line:
            #     line = (y := line.split(" "))[0]
            #     del y[0]
            #     lines += y
            code += "|"
            if line in opcode_keys:
                code += opcode_map[line]
                code += "|"
            else:
                if line.startswith("\""):
                    if line.endswith("\""):
                        str_typemap = type_map["str"]
                        code += str_typemap + \
                            hexlify(line.encode()).decode() + str_typemap
                    else:
                        raise Exception("Quote mismatch.")
                elif line.isdigit():
                    int_typemap = type_map["int"]
                    code += int_typemap + hex(int(line))[2:] + int_typemap
                elif line.startswith("0x"):
                    int_typemap = type_map["int"]
                    code += int_typemap + line[2:] + int_typemap
                else:
                    shallraise = False
                    try:
                        double_typemap = type_map["double"]
                        code += double_typemap + \
                            str(int(float(line) * 1e16)) + double_typemap
                    except ValueError:
                        shallraise = True
                    if shallraise:
                        raise Exception(f"Unknown keyword: {line}")

            code += "|"

        with open(file_out, "wb+") as f:
            dsettings['recurse'] = True
            f.write(hexlify(dumps(
                {
                    "bytecode": hexlify(code.encode()).decode(),
                    "lyra": hexlify(pdumps(self, HIGHEST_PROTOCOL)).decode()
                }
            ).encode()))
            # f.write(pickle.dumps({"execution_map": self.execution_map, "opcode_map": self.opcode_map}))
            # f.write(hexlify(code.encode()))

    def compute(self, stack, j, execution_map):
        compute_stack = stack[j:]

        i = 0
        while (True):
            if str(compute_stack[i]) in execution_map:

                func = execution_map[compute_stack[i]]["func"]
                argc = execution_map[compute_stack[i]]["args"]

                stack_i = compute_stack[i-argc:i]

                res = func(compute_stack, *stack_i[:i][::-1])

                del compute_stack[i-argc:i+1]
                if res is not None:
                    if isinstance(res, list):
                        compute_stack = res
                    elif hasattr(res, "dest"):
                        for index, key in enumerate(stack):
                            if key == "0x4" and stack[index - 1] == res.dest:
                                stack = stack[:index - 1]
                                break
                    elif hasattr(res, "halt"):
                        stack = []
                    else:
                        compute_stack.insert(i-argc, res)
                break

            else:
                i += 1
                continue

        return stack[:j] + compute_stack

    @staticmethod
    def execute(file_in: str):
        with open(file_in) as f:
            execobj = loads(unhexlify(f.read()).decode())
            code = unhexlify(execobj["bytecode"]).decode().split("|")
            lyra = ploads(unhexlify(execobj["lyra"]))
        lyra.executeWithContent(code)

    def executeWithObj(self: object, file_in: str):
        with open(file_in) as f:
            execobj = loads(unhexlify(f.read()).decode())
            code = unhexlify(execobj["bytecode"]).decode().split("|")
            self.executeWithContent(code)

    def executeWithContent(self, code: str):
        type_map = Lyra.type_map

        stack = []
        for line in code:
            if line == "":
                continue
            if line.startswith("0x"):
                stack.append(line)
            elif line.startswith(i := type_map["int"]):
                stack.append(int(line.split(i)[1], 16))
            elif line.startswith(s := type_map["str"]):
                stack.append(unhexlify(line.split(
                    s)[1].encode()).decode()[1:-1])
            elif line.startswith(d := type_map["double"]):
                stack.append(int(line.split(d)[1]) / 1e16)
            else:
                raise Exception("Unknown instruction.")

        stack = stack[::-1]

        if (not stack):
            return

        execution_map = self.execution_map

        if execution_map[stack[-1]]["args"] < 1:
            raise Exception("Must begin with a function.")

        arg = 0
        while (i := len(stack)) != 0:
            # print(stack)
            cont = False
            for elem in (str(elem) for elem in stack):
                if elem.startswith("0x"):
                    cont = True
                    break
            if (not cont):
                break

            m = i
            ret = False
            while(str(stack[m - 1]) not in execution_map):
                if m == 0:
                    ret = True
                    break
                m -= 1

            if ret:
                break

            i = m

            arg = execution_map[stack[i - 1]]["args"]

            j = i - 1
            while arg != 0:
                if j == 0:
                    break
                if str(stack[j - 1]) in execution_map:
                    arg += execution_map[stack[j - 1]]["args"] - 1
                else:
                    arg -= 1
                j -= 1

            stack = self.compute(stack, j, execution_map)
