import mgf_rs_io
from matchms import Spectrum  # Ensure matchms is installed (pip install matchms)

# Count the number of spectra.
count = mgf_rs_io.count_spectra("tests/data/small.mgf")
print("Number of spectra:", count)

# Load spectra as matchms Spectrum objects.
spectra = mgf_rs_io.load_from_mgf("tests/data/small.mgf")
print("Loaded {} spectra.".format(len(spectra)))

# Inspect each spectrum's metadata.
for spec in spectra:
    # Verify that each object is an instance of matchms Spectrum.
    print(isinstance(spec, Spectrum))
    # Print the spectrum; the metadata should include the "params" key showing all parameters.
    print(spec)

for spec in spectra:
    print(spec.metadata)          # prints the entire metadata dictionary
    print(spec.metadata.get("params"))  # prints the params sub-dictionary

spectra[0].metadata

