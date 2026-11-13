# ECOGRAMA — architecture

`ecograma-core` = DSP without I/O: windows, STFT, masks, repair algorithms. The naive `dft_magnitude` is the *oracle* for faster code.
Planned crates: `ecograma-io` (WAV/FLAC), `ecograma-gui` (egui spectrogram view), `ecograma-clap` (plug-in).
Every algorithm gets a synthetic-signal test (sine, click, hum) so contributors do not need audio files.
