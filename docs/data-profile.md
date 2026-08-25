# Hồ sơ dữ liệu tên đường

## 1. Mục đích

Tài liệu này mô tả snapshot
[`gofa_vietnam_real_road_names.csv`](../gofa_vietnam_real_road_names.csv) được bổ
sung vào repository. Mục tiêu là:

- cung cấp căn cứ định lượng để chọn quy tắc mặc định;
- nhận diện vấn đề chất lượng dữ liệu trước khi thiết kế thuật toán;
- xác định cách dùng dữ liệu trong kiểm thử và benchmark;
- tạo baseline để so sánh khi dữ liệu được cập nhật.

Các thống kê không khẳng định mọi dòng đều là tên đường hợp lệ hoặc mọi tiền tố
đều cần bị viết tắt/loại bỏ.

## 2. Thông tin snapshot

| Thuộc tính | Giá trị |
|---|---|
| Tên tệp | `gofa_vietnam_real_road_names.csv` |
| Kích thước | khoảng 2,0 MiB |
| SHA-256 | `56b46e53ff159cc5dc1f08bb8b5626842cd882d52572fad7e8b7ddf13822b80e` |
| Encoding | UTF-8 có BOM |
| Ký tự xuống dòng | CRLF |
| Header | `road_name` |
| Số cột | 1 |
| Số bản ghi | 78.615, không tính header |
| Bản ghi CSV sai số cột | 0 |
| Giá trị rỗng sau trim | 0 |
| Giá trị thay đổi khi trim hai đầu | 0 |
| Giá trị duy nhất theo byte | 78.615 |
| Giá trị duy nhất sau case folding | 77.962 |

Checksum dùng để gắn kết quả phân tích với đúng snapshot. Khi tệp thay đổi, cần
chạy lại profiling và cập nhật checksum cùng các bảng thống kê.

## 3. Phương pháp phân tích

- Đọc bằng CSV parser với encoding `utf-8-sig`, không tách trực tiếp theo dòng.
- Không thay đổi hay chuẩn hóa nội dung trước khi thống kê, ngoại trừ phép
  `casefold` ở thống kê được ghi rõ.
- Prefix được so khớp không phân biệt hoa/thường tại đầu chuỗi.
- Sau prefix phải là cuối chuỗi, whitespace hoặc một dấu phân cách đã chọn.
- Khi nhiều prefix khớp, prefix dài nhất thắng.
- Số lượng trong bảng phân bố là độc quyền, không đếm một dòng vào nhiều nhóm.

Đây là logic khám phá dữ liệu, chưa phải contract cuối cùng của crate. Đặc biệt,
việc chấp nhận dấu câu làm ranh giới cần được thu hẹp trong triển khai chính thức.

## 4. Tổng quan chất lượng

| Chỉ báo | Số bản ghi | Tỷ lệ |
|---|---:|---:|
| Khớp một tiền tố ứng viên | 63.424 | 80,68% |
| Không khớp tập tiền tố ứng viên | 15.191 | 19,32% |
| Bắt đầu bằng chữ số | 123 | 0,16% |
| Chứa ký tự chữ ngoài Latin | 74 | 0,09% |
| Chứa chữ Hán/CJK | 66 | 0,08% |
| Chỉ chứa ký hiệu, không có chữ hoặc số | 4 | <0,01% |

Có 630 nhóm tên trùng nhau sau khi bỏ khác biệt hoa/thường, tạo ra 653 dòng dư
so với tập duy nhất theo `casefold`. Ví dụ:

- `ĐƯỜNG NỘI BỘ`, `Đường Nội Bộ`, `Đường nội bộ`, `đường Nội Bộ`;
- `Cầu Vượt Đường Sắt`, `Cầu vượt Đường Sắt`, `Cầu vượt Đường sắt`,
  `Cầu vượt đường sắt`;
- `1 Tháng 5`, `1 tháng 5`.

Do đó, “duy nhất theo byte” không đồng nghĩa với “duy nhất về mặt ngữ nghĩa”.
Thư viện chỉ chuẩn hóa tiền tố, không nên tự động deduplicate toàn bộ tên.

## 5. Phân bố tiền tố ứng viên

Các nhóm dưới đây loại trừ lẫn nhau theo nguyên tắc tiền tố dài nhất thắng:

| Tiền tố | Số bản ghi | Tỷ lệ toàn bộ |
|---|---:|---:|
| `hẻm` | 28.211 | 35,89% |
| `ngõ` | 12.000 | 15,26% |
| `đường` | 11.489 | 14,61% |
| `ngách` | 5.688 | 7,24% |
| `phố` | 3.010 | 3,83% |
| `kiệt` | 1.207 | 1,54% |
| `đường tỉnh` | 892 | 1,13% |
| `quốc lộ` | 344 | 0,44% |
| `đường huyện` | 141 | 0,18% |
| `đường cao tốc` | 113 | 0,14% |
| `ĐT.` | 86 | 0,11% |
| `đại lộ` | 49 | 0,06% |
| `tỉnh lộ` | 40 | 0,05% |
| `đường vành đai` | 37 | 0,05% |
| `QL` | 25 | 0,03% |
| `ĐT` | 19 | 0,02% |
| `TL` | 18 | 0,02% |
| `vành đai` | 17 | 0,02% |
| `đường liên xã` | 12 | 0,02% |
| `QL.` | 7 | <0,01% |
| `đường liên thôn` | 6 | <0,01% |
| `huyện lộ` | 5 | <0,01% |
| `cao tốc` | 3 | <0,01% |
| `HL` | 2 | <0,01% |
| `xa lộ` | 2 | <0,01% |
| `TL.` | 1 | <0,01% |

Một số kết luận trực tiếp:

1. `Hẻm`, `Ngõ`, `Ngách` và `Kiệt` chiếm tỷ trọng lớn, nên không thể thiết kế bộ
   mặc định chỉ quanh `Đường` và `Phố`.
2. Dạng đầy đủ và dạng viết tắt cùng tồn tại (`Quốc lộ`/`QL`/`QL.`, `Đường
   tỉnh`/`ĐT`/`ĐT.`). Alias là yêu cầu thực tế để bảo đảm idempotency.
3. Prefix ghép phải được thử trước prefix tổng quát: `đường cao tốc`, `đường
   tỉnh`, `đường huyện` trước `đường`.
4. Các nhóm `đường tỉnh` và `tỉnh lộ`, hoặc `đường huyện` và `huyện lộ`, có thể
   tương đương về nghiệp vụ nhưng không nên gộp nếu chưa có phê duyệt.

## 6. Vấn đề dữ liệu quan sát được

### 6.1. Lỗi chính tả hoặc dấu

Số lần xuất hiện ở token đầu:

| Token | Số lượng | Nhận định sơ bộ |
|---|---:|---|
| `Hẻm` | 28.211 | Dạng chuẩn ứng viên |
| `Hèm` | 96 | Có khả năng sai dấu |
| `Hem` | 25 | Có khả năng thiếu dấu |
| `Hẽm` | 12 | Có khả năng sai dấu |
| `Đường` | 12.689 | Gồm cả prefix ghép |
| `Dường` | 11 | Có khả năng thiếu gạch ngang chữ `Đ` |
| `Đuờng` | 6 | Có khả năng sai vị trí dấu Unicode/chính tả |
| `Phố` | 3.010 | Dạng chuẩn ứng viên |
| `Phó` | 3 | Có thể là tên riêng hoặc lỗi dấu |

Theo quyết định nghiệp vụ, các biến thể sai dấu đã quan sát trong bảng được đưa
vào alias hard-code và chỉ được sửa khi xuất hiện ở đầu chuỗi tại ranh giới
prefix. Không áp dụng phép sửa gần đúng tổng quát cho phần tên còn lại.

### 6.2. Giá trị không phải tên đường sạch

Dữ liệu có các giá trị chỉ chứa ký hiệu như `'`, `--`, `\\`, `]`; bản ghi đầu
tiên còn giữ dấu nháy kép như một phần của giá trị:

```text
"Đường Mươi Phèo -Tư Cò"
```

Có 123 bản ghi bắt đầu bằng chữ số, ví dụ số ngõ/địa chỉ hoặc tên dạng ngày
tháng. Một số tên có prefix ở giữa như:

```text
1/108/3/24 Đường Phạm Như Xương
```

Matcher neo đầu chuỗi phải giữ nguyên các dòng này. Việc bóc số địa chỉ/ngõ trước
khi chuẩn hóa tên đường thuộc một tầng tiền xử lý khác.

### 6.3. Dữ liệu đa ngôn ngữ

Có 74 bản ghi chứa chữ ngoài Latin, gồm tên biên giới song ngữ và tên hoàn toàn
bằng chữ Hán hoặc Khmer. Ví dụ có dạng tên tiếng Việt kèm tên cầu bằng chữ Hán.

Thư viện phải an toàn với mọi chuỗi UTF-8 và giữ nguyên phần không khớp. Không
được giả định mỗi ký tự dài một byte hoặc toàn bộ dữ liệu là tiếng Việt Latin.

### 6.4. Khoảng trắng

Dữ liệu hiện không có whitespace ở hai đầu, nhưng có một số dòng chứa nhiều
whitespace ngay sau prefix. Theo quyết định nghiệp vụ, input passthrough hoặc
không khớp phải giữ nguyên whitespace. Chỉ rule thực sự thay/xóa prefix mới được
chuẩn hóa khoảng trắng tại ranh giới với remainder; không co khoảng trắng ở nơi
khác.

## 7. Ảnh hưởng tới thiết kế rule

Mô hình `RuleSpec` nội bộ nên phân biệt các khái niệm:

```rust,ignore
RuleSpec {
    canonical: "đường tỉnh",
    aliases: &["ĐT", "ĐT.", "DT"],
    abbreviate: AbbreviateAction::Replace("ĐT."),
    remove: RemoveAction::NormalizeCode("ĐT."),
}
```

- `prefix`: dạng đầy đủ dùng để nhận diện;
- `canonical`: cách viết đầy đủ chuẩn khi `RemoveAction::Keep` cần sửa alias;
- `abbreviate`: giữ nguyên hoặc thay bằng dạng viết tắt trong `Mode::Abbreviate`;
- `remove`: hành vi riêng trong `Mode::Remove`;
- `alias`: các dạng hợp lệ tương đương, không phải lỗi chính tả tùy đoán.

Builder cần phát hiện xung đột trên cả prefix và alias sau canonicalization.
Ví dụ, một alias không được ánh xạ đồng thời tới `đường tỉnh` và một rule khác.

## 8. Cách sử dụng dữ liệu trong dự án

### 8.1. Không nhúng toàn bộ vào crate mặc định

Tệp khoảng 2 MiB không cần thiết ở runtime nếu crate chỉ chứa thuật toán và rule.
Nhúng dữ liệu bằng `include_str!` sẽ làm tăng artifact và trộn dữ liệu tham chiếu
với public API. Chỉ nên đóng gói nếu có yêu cầu tra cứu trực tiếp được phê duyệt.

### 8.2. Sinh fixture có kiểm soát

Không dùng chính kết quả của thuật toán làm expected output vì sẽ tạo kiểm thử
tự xác nhận. Quy trình đề xuất:

1. Lấy mẫu phân tầng theo prefix, alias và nhóm không khớp.
2. Thêm các trường hợp biên: số ở đầu, Unicode, ký hiệu, prefix dính liền.
3. Chuyên gia nghiệp vụ duyệt expected output.
4. Lưu fixture nhỏ, ổn định trong `tests/fixtures`.
5. Dùng toàn bộ tệp cho test phân tích/regression tùy chọn, không cho unit test
   bắt buộc của downstream.

### 8.3. Đo coverage, không chỉ pass/fail

Một công cụ phân tích ngoại tuyến nên báo ít nhất:

- tổng bản ghi;
- tỷ lệ khớp theo rule;
- tỷ lệ không khớp;
- xung đột alias;
- số output rỗng;
- số bản ghi thay đổi;
- các token đầu chưa được nhận diện phổ biến nhất.

Không tự động thêm rule chỉ vì token xuất hiện nhiều. Các token như `Cầu`, `Cầu
vượt`, `Hầm`, `Xóm` đã được chốt là passthrough: giữ nguyên trong cả hai mode và
chỉ áp dụng quy tắc viết hoa chữ cái đầu chung.

## 9. Tiêu chí cập nhật dữ liệu

Mỗi lần thay snapshot:

1. kiểm tra CSV có đúng một cột `road_name`;
2. xác nhận encoding và số bản ghi;
3. tính lại SHA-256;
4. chạy lại phân bố prefix và báo chênh lệch so với snapshot trước;
5. review các token đầu mới hoặc tăng đột biến;
6. không cập nhật rule mặc định nếu chưa có fixture và phê duyệt nghiệp vụ;
7. ghi nguồn, thời điểm trích xuất và giấy phép/quyền sử dụng dữ liệu.

## 10. Thông tin còn thiếu

Snapshot chưa đi kèm metadata về:

- nguồn hệ thống và thời điểm trích xuất;
- phạm vi địa lý chính xác;
- ý nghĩa của “tất cả tên đường hiện có” và mức độ bao phủ;
- quy trình deduplicate;
- giấy phép/quyền phân phối;
- tần suất cập nhật;
- trường hợp nào được xem là dữ liệu hợp lệ.

Các thông tin này cần được bổ sung trước khi sử dụng tệp làm dữ liệu phát hành
hoặc căn cứ chính thức cho SLA chất lượng.
