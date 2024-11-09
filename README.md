# VBT
[![Weekly Run](https://github.com/Irilith/VBT/actions/workflows/daily.yml/badge.svg)](https://github.com/Irilith/VBT/actions/workflows/daily.yml)
[![CI](https://github.com/Irilith/VBT/actions/workflows/ci.yml/badge.svg)](https://github.com/Irilith/VBT/actions/workflows/ci.yml)

Dự án này tạo ra một RSS feed để theo dõi việc đăng ký xuất bản của những cuốn sách có bản quyền ở Việt Nam. Nó cung cấp các cập nhật mỗi 7 ngày về các cuốn sách mới được đăng ký (nếu có).

## Cách dùng

Bạn có thể dùng bất kỳ RSS feed reader nào mà bạn thích (chắc thế), riêng mình thì mình dùng thunderbird, Link rss thì các bạn vào /feed/rss/{title} và chọn bộ mà muốn theo dõi, nhấn raw, copy cái link đó và paste vào RSS Feed Tracker của bạn

Ví dụ: https://raw.githubusercontent.com/Irilith/VBT/refs/heads/main/feed/rss/Majo_no_Tabitabi.rss

## Tự Hosting

Nếu bạn muốn tự host dự án này, cách khuyến khích là fork repo này và bật GitHub Actions trong fork của bạn.

## Thiết Lập Secrets

Hãy nhớ thiết lập secrets cho webhook `"WEEKLY"` và `"PROCESSED"`. Nếu không, bạn sẽ gặp phải lỗi khi chạy action (mình sẽ sửa sau, hiện tại hơi lười).

## Đề Xuất Sách Bạn Muốn Theo Dõi

Nếu bạn muốn đề xuất một cuốn sách để theo dõi, bạn có thể mở một issue hoặc pull request nếu bạn biết cách thực hiện. Ngoài ra, bạn cũng có thể đề xuất sách trực tiếp trong Discord server của mình. Cuốn sách phải là sách nổi tiếng (dựa trên số lượt vote của issue hoặc pull request, hoặc dựa trên hiểu biết của mình) hoặc ít nhất là cuốn sách mà mình muốn theo dõi.

## Tham Gia Discord Server

Nếu bạn muốn trò chuyện, hãy tham gia server Discord của mình: [discord.gg](https://discord.gg/VJ57nka8G6)
