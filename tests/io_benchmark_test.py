import time
import mgf_rs_io
from matchms.importing import load_from_mgf

# Benchmark the new Rust-based loader.
start_time = time.time()
spectra_rust = list(mgf_rs_io.load_from_mgf("tests/data/small.mgf"))
rust_time = time.time() - start_time
print("mgf_rs_io.load_from_mgf:")
print("  Time: {:.4f} sec".format(rust_time))
print("  Number of spectra:", len(spectra_rust))

# Benchmark the legacy matchms loader.
start_time = time.time()
spectra_legacy = list(load_from_mgf("tests/data/small.mgf"))
legacy_time = time.time() - start_time
print("\nmatchms.load_from_mgf:")
print("  Time: {:.4f} sec".format(legacy_time))
print("  Number of spectra:", len(spectra_legacy))
