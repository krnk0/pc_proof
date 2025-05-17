import argparse
from sequent import prove
from tableau import solve
from viz import prove_svg


def main(argv=None):
    parser = argparse.ArgumentParser(description="Propositional logic prover")
    parser.add_argument("expr", help="expression to prove")
    parser.add_argument(
        "-m",
        "--method",
        choices=["sequent", "tableau"],
        default="sequent",
        help="proof method to use",
    )
    parser.add_argument(
        "-o",
        "--output",
        help="write sequent proof as SVG to given path (without extension)",
    )
    args = parser.parse_args(argv)

    if args.method == "sequent":
        if args.output:
            res = prove_svg(args.expr, args.output)
            print("Provable" if res else "Not provable")
            print(f"SVG saved to {args.output}.svg")
        else:
            res = prove(args.expr)
            print("Provable" if res else "Not provable")
    else:
        if args.output:
            parser.error("SVG output is only supported with the sequent method")
        taut, model = solve(args.expr, tautology=True)
        if taut:
            print("Provable")
        else:
            print("Not provable")
            if model:
                print("Counterexample:", model)


if __name__ == "__main__":
    main()
