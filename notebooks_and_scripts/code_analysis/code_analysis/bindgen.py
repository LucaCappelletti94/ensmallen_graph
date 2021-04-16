import re
import json
from ensmallen_graph import EnsmallenGraph
from .utils import build_path


def bindgen(args):
    with open(build_path("results/analysis.json"), "r") as f:
        functions = json.load(f)

    bindgen = ""
    bindgen += "use super::*;\n"
    bindgen += "impl Graph {\n"

    for function in functions:
        if function.get("modifiers", "") != "pub":
            continue

        if function.get("struct", "") != "Graph":
            continue

        if "name" not in function.keys():
            print("WTF", function)
            continue

        #if function["name"] in dir(EnsmallenGraph):
        #    continue

        if "iter" in function["name"]:
            continue

        result = ""

        if len(function.get("args", [])) > 1:
            signature = ", " + ", ".join(
                x[0]
                for x in function.get("args", [])[1:]
            )
        else:
            signature = ""

        result += "#[text_signature = \"($self{signature})\"]\n".format(
            signature=signature
        )

        result += "/// TODO!: This binding was automatically generated\n"
        if "doc" in function:
            doc = "\n".join(function.get("doc", []))
            # Remove examples
            doc = re.sub("```.+```", "", doc, flags=re.DOTALL)
            # Remove example header
            doc = re.sub("# Example[^#]+", "", doc, flags=re.DOTALL)
            # Convert the arguments header
            doc = re.sub("#\s+Arguments", "Paramenters\n--------------", doc, flags=re.DOTALL)
            # Convert the arguments in python format
            doc = re.sub(r"[ \t]*\*`?(.+?)`?\s*:\s*(.+?)\s*-\s*(.+)", r"\1 : \2,\n\t\3", doc)
            # Type conversions
            doc = re.sub(r"Vec<(.+?)>", r"List[\1]", doc)
            doc = re.sub(r"EdgeTypeT", r"int", doc)
            doc = re.sub(r"NodeTypeT", r"int", doc)
            doc = re.sub(r"String", r"str", doc)
            doc = re.sub(r"NodeT", r"int", doc)
            doc = re.sub(r"EdgeT", r"int", doc)
            doc = re.sub(r"&", r"", doc)
            doc = re.sub(r"Option<(.+?)>", r"\1", doc)
            doc = re.sub(r"HashSet<(.+?)>", r"Dict[\1]", doc)
            # Remove white space at the edges
            doc = doc.strip()
            result += "\n".join("/// " + x for x in doc.split("\n"))
            result += "\n"

        if len(function.get("args", [])) > 1:
            args = function["args"][0][1] + ", " + ", ".join([
                "%s : %s"%tuple(x) 
                for x in function["args"][1:]
            ])
        else:
            args = function["args"][0][1]

        result += "fn {name}({args})".format(
            name=function["name"],
            args=args
        )

        if "return_type" in function:
            result_type = function["return_type"]
            if result_type.startswith("Result"):
                result_type = "Py" + result_type
                result_type, _, _ = result_type.rpartition(",")
                result_type += ">"

            result += " -> %s "%result_type
        
        body = "\tself.graph." + function["name"]
        body += "(" 
        body += ", ".join(x[0] for x in function["args"][1:])
        body += ")"

        if "return_type" in function:
            result_type = function["return_type"]
            if result_type.startswith("Result"):
                body = "\tpe!(%s)"%body[1:]

        result += "{\n"
        result += body
        result += "\n}\n"

        bindgen += "\n\t" + "\n\t".join(result.split("\n"))

        print(result)
        print("#"*60)

    bindgen += "\n}\n"

    with open(build_path("results/bindgen.rs"), "w") as f:
        f.write(bindgen)