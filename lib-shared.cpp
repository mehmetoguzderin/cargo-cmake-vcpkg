#include <iostream>
#include <tiffio.h>

extern "C" {
bool writeTiff(const char *filename, const uint8_t *data, int width,
               int height) {
  TIFF *tif = TIFFOpen(filename, "w");
  if (tif == NULL) {
    std::cerr << "Can't open " << filename << " for writing" << std::endl;
    return false;
  }

  TIFFSetField(tif, TIFFTAG_IMAGEWIDTH, width);
  TIFFSetField(tif, TIFFTAG_IMAGELENGTH, height);
  TIFFSetField(tif, TIFFTAG_SAMPLESPERPIXEL, 1);
  TIFFSetField(tif, TIFFTAG_BITSPERSAMPLE, 8);
  TIFFSetField(tif, TIFFTAG_ORIENTATION, ORIENTATION_TOPLEFT);
  TIFFSetField(tif, TIFFTAG_PLANARCONFIG, PLANARCONFIG_CONTIG);
  TIFFSetField(tif, TIFFTAG_PHOTOMETRIC, PHOTOMETRIC_MINISBLACK);

  for (int i = 0; i < height; i++) {
    if (TIFFWriteScanline(tif, (void *)(data + i * width), i, 0) < 0) {
      std::cerr << "Can't write image to " << filename << std::endl;
      TIFFClose(tif);
      return false;
    }
  }

  TIFFClose(tif);
  return true;
}
}
