import sys, os, json, time


from bioiain import log
import mmcif_parser


def parse_rust(filepath, verbose=False):
    from mmcif_parser import read_mmcif
    start = time.time_ns()
    data = read_mmcif(filepath, as_dict=True, subset=[], verbose=verbose)
    end = time.time_ns()
    nanoseconds = end-start
    seconds = nanoseconds / 1000000000
    log(1, f"Elapsed seconds: {seconds:2.5f}")
    log(1, f"N keys: {len(data.keys())}")
    return data

def parse_bioiain(filepath, verbose=False):
    from bioiain.base.mmcif import read_mmcif

    start = time.time_ns()
    data = read_mmcif(filepath, as_dict=True, subset=None, verbose=verbose)
    end = time.time_ns()
    nanoseconds = end-start
    seconds = nanoseconds / 1000000000
    log(1, f"Elapsed seconds: {seconds:2.5f}")
    log(1, f"N keys: {len(data.keys())}")
    return data


if __name__ == "__main__":
    log("header", "Benchmarking mmcif parsers")
    filepath = "./test-cases/1M2Z.cif"


    mode = None
    if "--bioiain" in sys.argv:
        mode="bioiain"
    elif "--rust" in sys.argv:
        mode="rust"
    if mode is None:
        log(1, filepath)
        log("header", "bioiain")
        parse_bioiain(filepath)
        log("header", "rust")
        parse_rust(filepath)
    elif mode in ["rust", "bioiain"]:
        if mode == "rust":
            log("header", "rust")
            data = parse_rust(filepath, verbose=False)
        elif mode == "bioiain":
            log("header", "bioiain")
            data = parse_bioiain(filepath, verbose=False)
        print(json.dumps(data, indent=4))


