from io import BufferedReader
import time
import can
import click

DME1 = 0x316


def parse_blf(file: BufferedReader) -> list[can.Message]:
    reader = can.io.blf.BLFReader(file)
    return [msg for msg in reader]


@click.option(
    "--input",
    "-i",
    type=click.File(mode="rb"),
    help="Input blf file",
    default="sabine.blf",
)
@click.command()
def cli(input):
    """Simple script to send DME1 messages on bus for testing
    """
    bus = can.Bus(interface="socketcan", channel="can0")
    for msg in parse_blf(input):
        if msg.arbitration_id == DME1:
            bus.send(msg)
            time.sleep(0.5)


if __name__ == "__main__":
    cli()
