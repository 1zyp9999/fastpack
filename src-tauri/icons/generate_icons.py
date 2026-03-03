#!/usr/bin/env python3
"""生成 RGBA PNG 图标"""
import struct
import zlib

def create_rgba_png(width, height, color=(102, 126, 234)):
    """创建带 alpha 通道的 PNG"""
    def png_chunk(chunk_type, data):
        chunk_len = struct.pack('>I', len(data))
        chunk_crc = struct.pack('>I', zlib.crc32(chunk_type + data) & 0xffffffff)
        return chunk_len + chunk_type + data + chunk_crc
    
    # PNG 签名
    signature = b'\x89PNG\r\n\x1a\n'
    
    # IHDR 块（RGBA 格式）
    ihdr_data = struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)  # 6 = RGBA
    ihdr = png_chunk(b'IHDR', ihdr_data)
    
    # IDAT 块（图像数据）
    raw_data = b''
    for y in range(height):
        raw_data += b'\x00'  # 过滤器类型
        for x in range(width):
            # 渐变效果
            factor = 1 - (y / height) * 0.3
            raw_data += bytes([
                int(color[0] * factor),
                int(color[1] * factor),
                int(color[2] * factor),
                255  # Alpha
            ])
    
    compressed = zlib.compress(raw_data, 9)
    idat = png_chunk(b'IDAT', compressed)
    
    # IEND 块
    iend = png_chunk(b'IEND', b'')
    
    return signature + ihdr + idat + iend

# 生成图标
sizes = [(32, '32x32.png'), (128, '128x128.png'), (256, '128x128@2x.png'), (256, 'icon.png')]
for size, filename in sizes:
    png_data = create_rgba_png(size, size)
    with open(filename, 'wb') as f:
        f.write(png_data)
    print(f'Created: {filename}')

print('Done!')
