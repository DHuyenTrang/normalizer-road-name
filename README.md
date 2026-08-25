# Road Name Normalizer

Thư viện Rust dùng chung để chuẩn hóa tên đường theo một tập quy tắc có thể cấu
hình, chẳng hạn:

- viết tắt tiền tố: `Đường Nguyễn Trãi` → `Đ. Nguyễn Trãi`;
- loại bỏ tiền tố: `Đường Nguyễn Trãi` → `Nguyễn Trãi`;
- chuẩn hóa khoảng trắng và cách viết tiền tố;
- quản lý tập quy tắc tập trung, được biên dịch trực tiếp cùng crate.

Phiên bản `0.1.0` có crate Rust, 19 quy tắc mặc định và bộ kiểm thử nghiệp vụ.
Xem [Tài liệu phân tích và thiết kế](docs/analysis-design.md) để biết các quyết
định kỹ thuật và [Hồ sơ dữ liệu](docs/data-profile.md) để biết các thống kê từ
danh sách tên đường hiện có.

## Dữ liệu tham chiếu

Repository có tệp [`gofa_vietnam_real_road_names.csv`](gofa_vietnam_real_road_names.csv)
gồm 78.615 tên đường duy nhất theo byte. Tệp này được dùng để khám phá quy tắc,
tạo fixture và đo độ bao phủ; không nên coi mọi dòng là một tên đường đã chuẩn
hóa hoặc dùng toàn bộ tệp làm unit test.

Phân tích snapshot hiện tại cho thấy 80,68% bản ghi bắt đầu bằng một trong các
tiền tố ứng viên. Các nhóm lớn nhất là `Hẻm`, `Ngõ`, `Đường`, `Ngách` và `Phố`.
Dữ liệu cũng có lỗi chính tả, chuỗi chỉ chứa ký hiệu, tên bắt đầu bằng số và ký
tự ngoài Latin. Vì vậy, thuật toán phải giữ nguyên dữ liệu không khớp thay vì cố
đoán hoặc sửa tự động.

Chi tiết về encoding, checksum, phân bố tiền tố và các vấn đề chất lượng nằm tại
[Hồ sơ dữ liệu](docs/data-profile.md).

## Phạm vi

Thư viện tập trung vào việc biến đổi **tiền tố loại đường** ở đầu chuỗi. Phần tên
riêng còn lại được giữ nguyên tối đa.

Trong phạm vi:

- nhận diện tiền tố không phân biệt chữ hoa/chữ thường;
- hỗ trợ tiếng Việt có dấu và Unicode;
- viết tắt, loại bỏ hoặc giữ nguyên tiền tố;
- chuẩn hóa khoảng trắng ở ranh giới giữa tiền tố và tên riêng;
- sử dụng một bộ quy tắc cố định, được review và version cùng mã nguồn;
- xử lý an toàn khi đầu vào rỗng hoặc không khớp quy tắc;
- viết hoa chữ cái đầu của mọi output;
- chuẩn hóa mã đường dính số, ví dụ `QL2` → `QL.2`, `ĐT261D` → `ĐT.261D`.

Ngoài phạm vi ban đầu:

- sửa chính tả tổng quát hoặc suy đoán tên đường bị thiếu; thư viện chỉ sửa các
  biến thể sai dấu của tiền tố đã được duyệt;
- chuẩn hóa toàn bộ chữ hoa/chữ thường của tên riêng;
- tách địa chỉ đầy đủ thành số nhà, phường/xã, quận/huyện;
- phiên âm hoặc loại bỏ dấu tiếng Việt;
- thay thế cụm từ xuất hiện ở giữa tên đường.

## Hành vi mong muốn

| Đầu vào | Chế độ | Kết quả |
|---|---|---|
| `Đường Nguyễn Trãi` | `Abbreviate` | `Đ. Nguyễn Trãi` |
| `đường   Nguyễn Trãi` | `Abbreviate` | `Đ. Nguyễn Trãi` |
| `Phố Huế` | `Remove` | `Huế` |
| `Quốc lộ 1A` | `Abbreviate` | `QL. 1A` |
| `Hẻm 12 Đường Số 1` | `Abbreviate` | `Hẻm 12 Đường Số 1` |
| `Kiệt 83 Nguyễn Duy Hiệu` | `Abbreviate` | `Kiệt 83 Nguyễn Duy Hiệu` |
| `Nguyễn Trãi` | bất kỳ | `Nguyễn Trãi` |
| `nguyễn trãi` | bất kỳ | `Nguyễn trãi` |
| `Đường` | `Remove` | chuỗi rỗng |

Bảng trên minh họa hành vi. Danh sách 19 quy tắc mặc định được version tại
[`src/rules.rs`](src/rules.rs) và mô tả trong
[`docs/rules-catalog.md`](docs/rules-catalog.md).

## API

Tên crate là `road_name_normalizer`.

```rust,no_run
use road_name_normalizer::{normalize, Mode};

fn main() {
    let abbreviated = normalize("Đường Nguyễn Trãi", Mode::Abbreviate);
    assert_eq!(abbreviated, "Đ. Nguyễn Trãi");

    let removed = normalize("Phố Huế", Mode::Remove);
    assert_eq!(removed, "Huế");
}
```

API cố ý nhỏ:

- `Mode`: `Abbreviate` hoặc `Remove`;
- `normalize(input, mode)`: nhận một cụm từ và trả về kết quả tương ứng;
- rule không nhận từ runtime; chúng nằm trong code và được version cùng crate.

Trong `Mode::Remove`, mỗi loại có hành vi đã duyệt riêng. Một số tiền tố được
xóa, một số được giữ nguyên hoặc chuẩn hóa để không làm mất ý nghĩa; danh mục
rule là nguồn chuẩn cho hành vi này.

Xem [Bảng rule đề xuất](docs/rules-catalog.md) để review trực tiếp input prefix,
output rút gọn và output xóa bỏ.

## Quy tắc chuẩn hóa

Pipeline gồm các bước sau:

1. Giữ nguyên input gốc; không trim hoặc co khoảng trắng toàn cục.
2. Tìm rule hard-code khớp ở **đầu chuỗi** và tại ranh giới từ hợp lệ.
3. So khớp không phân biệt chữ hoa/chữ thường.
4. Nếu có nhiều quy tắc khớp, ưu tiên tiền tố dài hơn; nếu cùng độ dài, ưu
   tiên quy tắc được khai báo trước.
5. Áp dụng chế độ viết tắt, loại bỏ hoặc giữ nguyên.
6. Chỉ chuẩn hóa khoảng trắng tại ranh giới do một rule biến đổi; không thay đổi
   phần tên riêng còn lại.
7. Viết hoa chữ cái đầu của output cuối cùng.

Ví dụ, quy tắc `đường` không được khớp với `Đườngsắt`, và quy tắc `quốc lộ`
được ưu tiên hơn một quy tắc tổng quát `quốc`.

## Tích hợp

Khi crate đã được triển khai, có thể dùng dependency theo đường dẫn trong
workspace:

```toml
[dependencies]
road_name_normalizer = { path = "../road-name-normalizer" }
```

Sau khi phát hành nội bộ hoặc công khai, thay `path` bằng nguồn Git, registry và
phiên bản cụ thể. Không nên dùng nhánh động như `main` trong môi trường production.

## Yêu cầu chất lượng

Trước phiên bản `0.1.0`, dự án cần đạt:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --no-deps
```

Ngoài unit test, bộ kiểm thử nên có bảng dữ liệu nghiệp vụ bao gồm chữ hoa/chữ
thường, dấu tiếng Việt, khoảng trắng, chuỗi rỗng, tiền tố chồng lấn và các tên
đường không có tiền tố.

Audit toàn bộ snapshot và benchmark có thể chạy riêng:

```bash
cargo test reference_snapshot_audit --release -- --ignored --nocapture
cargo bench --bench normalize
```

## Cấu trúc dự án

```text
.
├── Cargo.toml
├── README.md
├── docs/
│   ├── analysis-design.md
│   ├── data-profile.md
│   └── rules-catalog.md
├── src/
│   ├── lib.rs
│   ├── normalizer.rs
│   └── rules.rs
└── tests/
    ├── fixtures/
    └── normalization.rs
```

## Phiên bản và tương thích

Dự án nên tuân theo Semantic Versioning. Trước `1.0.0`, thay đổi API vẫn có thể
xảy ra; mọi thay đổi về kết quả chuẩn hóa cần được xem là thay đổi hành vi và
ghi rõ trong changelog. MSRV của phiên bản `0.1.0` là Rust 1.81.

## Đóng góp

Khi thêm hoặc sửa một quy tắc:

1. mô tả nguồn và mục đích nghiệp vụ của quy tắc;
2. thêm ca kiểm thử cho dạng đầy đủ, chữ hoa/chữ thường và trường hợp không được
   khớp;
3. kiểm tra xung đột với tiền tố ngắn hơn hoặc dài hơn;
4. không thay đổi phần tên riêng nếu yêu cầu chỉ liên quan đến tiền tố.

## Giấy phép

Chưa xác định. Cần bổ sung tệp `LICENSE` trước khi phân phối crate cho bên ngoài
tổ chức.
