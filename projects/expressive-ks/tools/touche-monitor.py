import rtmidi
import time

def list_ports(midi_in):
    ports = midi_in.get_ports()
    print("Available MIDI input ports:")
    for i, name in enumerate(ports):
        print(f"  [{i}] {name}")
    return ports

def main():
    midi_in = rtmidi.MidiIn()
    ports = list_ports(midi_in)

    index = int(input("Select Touche SE port number: "))
    midi_in.open_port(index)
    print(f"Monitoring port: {ports[index]}")
    print("Move the Touche SE on each axis, one at a time.")
    print("Press Ctrl+C to stop.\n")

    cc_ranges = {}

    try:
        while True:
            msg = midi_in.get_message()
            if msg:
                data, _ = msg
                if len(data) == 3 and (data[0] & 0xF0) == 0xB0:
                    cc = data[1]
                    val = data[2]
                    if cc not in cc_ranges:
                        cc_ranges[cc] = {"min": val, "max": val}
                    else:
                        cc_ranges[cc]["min"] = min(cc_ranges[cc]["min"], val)
                        cc_ranges[cc]["max"] = max(cc_ranges[cc]["max"], val)
                    print(f"CC{cc:3d}  val={val:3d}   range so far: "
                          f"{cc_ranges[cc]['min']}-{cc_ranges[cc]['max']}")
            time.sleep(0.001)
    except KeyboardInterrupt:
        print("\n--- Final CC ranges observed ---")
        for cc, r in sorted(cc_ranges.items()):
            print(f"  CC{cc}: min={r['min']}  max={r['max']}")

if __name__ == "__main__":
    main()
