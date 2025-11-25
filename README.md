# [PlutoFilter](https://github.com/sammycage/plutofilter) port to rust

## Gaussian Blur

Applies a Gaussian blur to the input surface using separable convolution. The amount of blur is controlled by the standard deviation along the horizontal and vertical axes. A value of `0` applies no blur.

| `0x0`                                                           | `5x5`                                                           | `10x10`                                                           |
| --------------------------------------------------------------- | --------------------------------------------------------------- | ----------------------------------------------------------------- |
| ![](res/test_output_images/gaussian_blur/gaussian_blur-0x0.png) | ![](res/test_output_images/gaussian_blur/gaussian_blur-5x5.png) | ![](res/test_output_images/gaussian_blur/gaussian_blur-10x10.png) |

## Color Transform

Applies a 5×4 color transformation matrix to each pixel in the input surface. The matrix operates on color and alpha channels, allowing both isolated and cross-channel transformations. The input and output surfaces may be the same for in-place filtering.

### Example

```c
const float original[20] = {
    1.0f, 0.0f, 0.0f, 0.0f, 0.0f,
    0.0f, 1.0f, 0.0f, 0.0f, 0.0f,
    0.0f, 0.0f, 1.0f, 0.0f, 0.0f,
    0.0f, 0.0f, 0.0f, 1.0f, 0.0f
};

const float grayscale[20] = {
    0.2126f, 0.7152f, 0.0722f, 0.0f, 0.0f,
    0.2126f, 0.7152f, 0.0722f, 0.0f, 0.0f,
    0.2126f, 0.7152f, 0.0722f, 0.0f, 0.0f,
    0.0f,    0.0f,    0.0f,    1.0f, 0.0f
};

const float sepia[20] = {
    0.393f, 0.769f, 0.189f, 0.0f, 0.0f,
    0.349f, 0.686f, 0.168f, 0.0f, 0.0f,
    0.272f, 0.534f, 0.131f, 0.0f, 0.0f,
    0.0f,   0.0f,   0.0f,   1.0f, 0.0f
};

const float contrast[20] = {
    1.75f, 0.0f,  0.0f,  0.0f, -0.375f,
    0.0f,  1.75f, 0.0f,  0.0f, -0.375f,
    0.0f,  0.0f,  1.75f, 0.0f, -0.375f,
    0.0f,  0.0f,  0.0f,  1.0f,   0.0f
};
```

| `original`                                               | `grayscale`                                               | `sepia`                                               | `contrast`                                               |
| -------------------------------------------------------- | --------------------------------------------------------- | ----------------------------------------------------- | -------------------------------------------------------- |
| ![](res/test_output_images/color_transform/original.png) | ![](res/test_output_images/color_transform/grayscale.png) | ![](res/test_output_images/color_transform/sepia.png) | ![](res/test_output_images/color_transform/contrast.png) |
