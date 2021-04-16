import re
import sys
import json
from ensmallen_graph import EnsmallenGraph
from .utils import build_path
from .parsers import DocParser

def doc_analysis(args):

    with open(build_path("results/analysis.json"), "r") as f:
        functions = json.load(f)

    result = {}

    for function in functions:
        p = DocParser()
        result[function.get("name", "")] = p.start(function, "\n".join(function["doc"]))

    print(json.dumps(result, indent=4))

    with open(build_path("results/doc_analysis.json"), "w") as f:
        json.dump(result, f, indent=4)