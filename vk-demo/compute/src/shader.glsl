#version 450

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer InputBuffer {
    uint data[];
} inputBuffer;

layout(set = 0, binding = 1) buffer OutputBuffer {
    uint data[];
} outputBuffer;

void main() {
    outputBuffer.data[0] = inputBuffer.data[0] + inputBuffer.data[1];
}
