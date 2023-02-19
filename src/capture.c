#include <CoreFoundation/CoreFoundation.h>
#include <CoreGraphics/CoreGraphics.h>
#include <CoreServices/CoreServices.h>
#include <ImageIO/ImageIO.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

enum CaptureResult {
  Ok,
  GetDisplayCountErr,
  GetDisplayErr,
  OutOfBoundsScreen,
  InvalidDisplay,
  PixelFormatErr,
  DimensionAndDataMismatchErr,
  CouldNotCreateDest,
  ImageWriteErr,
};

// NOTE: type CFIndex = long;
enum CaptureResult capture_to(const unsigned char *path, long path_length,
                              size_t screen) {
  // We need count before we can allocate an array for the active displays.
  uint32_t count;
  if (CGGetActiveDisplayList(0, 0, &count) != CGDisplayNoErr)
    return GetDisplayCountErr;

  CGDirectDisplayID *displays =
      (CGDirectDisplayID *)malloc(count * sizeof(CGDirectDisplayID));
  if (CGGetActiveDisplayList(count, displays, &count) != CGDisplayNoErr)
    return GetDisplayErr;

  if (screen >= (uintmax_t)count)
    return OutOfBoundsScreen;

  CGImageRef screenshot = CGDisplayCreateImage(displays[screen]);
  if (!screenshot)
    return InvalidDisplay;

  // TODO: Verify image data. w*h*type = raw_image. bits_per_pixel % 8 == 0.

  CFURLRef path_url = CFURLCreateWithBytes(
      kCFAllocatorDefault, path, path_length, kCFStringEncodingUTF8, 0);
  CGImageDestinationRef destination =
      CGImageDestinationCreateWithURL(path_url, kUTTypePNG, 1, 0);
  printf("%s\n", path);
  if (!destination)
    return CouldNotCreateDest;

  CGImageDestinationAddImage(destination, screenshot, 0);
  if (!CGImageDestinationFinalize(destination))
    return ImageWriteErr;

  CFRelease(destination);
  CFRelease(path_url);
  CFRelease(screenshot);
  free(displays);

  return Ok;
}
