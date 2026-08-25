# Bảng rule rút gọn và xóa tiền tố

## 1. Trạng thái

Đây là bảng rule của phiên bản `0.1.0`, tổng hợp từ snapshot 78.615 tên đường.
Các rule được khai báo trực tiếp trong `src/rules.rs`; crate không đọc cấu hình
runtime.

Public API:

```rust,ignore
pub fn normalize(input: &str, mode: Mode) -> String;
```

Hai mode đang yêu cầu:

- `Mode::Abbreviate`: thay tiền tố bằng dạng rút gọn;
- `Mode::Remove`: áp dụng hành vi xóa đã duyệt cho từng loại; một số loại được
  giữ nguyên hoặc chuẩn hóa thay vì xóa để không làm mất ý nghĩa tên đường.

## 2. Nguyên tắc áp dụng chung

1. Không trim hoặc co khoảng trắng toàn cục. Với input không khớp hoặc thuộc
   nhóm passthrough, giữ nguyên mọi ký tự và chỉ viết hoa chữ cái đầu tiên.
2. Unicode-normalize về NFC để nhận đúng cả dạng dựng sẵn và dạng dấu tổ hợp.
3. So khớp tiền tố không phân biệt hoa/thường.
4. Chỉ khớp ở đầu chuỗi; không thay từ xuất hiện giữa chuỗi.
5. Thử rule dài hơn trước: `đường cao tốc` trước `đường`.
6. Sau tiền tố dạng từ phải là cuối chuỗi hoặc Unicode whitespace.
7. Alias mã đường `QL`, `ĐT`, `DT`, `TL`, `HL`, `VĐ` được phép có dấu chấm
   và được phép dính liền với mã số, ví dụ `QL2`, `ĐT.204`, `DT518`.
8. Mã đường được chuẩn hóa có dấu chấm: `QL2` → `QL.2`, `ĐT261D` → `ĐT.261D`.
   Nếu input vốn có khoảng trắng thì giữ một khoảng trắng: `DT 258` → `ĐT. 258`.
9. Chuẩn hóa các biến thể tiền tố sai dấu đã xác định trong mục 7.
10. Khi một rule biến đổi prefix, chỉ chuẩn hóa khoảng trắng tại ranh giới giữa
    prefix mới và remainder theo rule đó.
11. Nếu không khớp rule, giữ nguyên input ngoài việc viết hoa chữ cái đầu.
12. Mọi output được viết hoa chữ cái đầu tiên; phần còn lại giữ nguyên tối đa.
13. Nếu input chỉ chứa một tiền tố có hành vi xóa, `Remove` trả chuỗi rỗng.

Ví dụ chuẩn hóa mã đường ở cả hai mode:

| Input | Output chuẩn hóa |
|---|---|
| `QL2` | `QL.2` |
| `ĐT261D` | `ĐT.261D` |
| `DT 258` | `ĐT. 258` |
| `TL4` | `TL.4` |
| `HL2` | `HL.2` |

## 3. Rule cho trường hợp rút gọn

`Input nhận diện` liệt kê canonical prefix và các alias hợp lệ. Dạng hoa/thường
không cần khai báo riêng.

| ID | Ưu tiên | Input nhận diện | Output tiền tố | Ví dụ input | Output rút gọn | Bằng chứng dữ liệu |
|---|---:|---|---|---|---|---:|
| A01 | 1 | `đường cao tốc` | `CT.` | `Đường cao tốc Biên Hoà - Vũng Tàu` | `CT. Biên Hoà - Vũng Tàu` | 113 |
| A02 | 2 | `đường vành đai` | `VĐ.` | `Đường Vành Đai 3.5` | `VĐ. 3.5` | 37 |
| A03 | 3 | `đường liên thôn` | Giữ nguyên | `Đường Liên Thôn 2` | `Đường Liên Thôn 2` | 6 |
| A04 | 4 | `đường liên xã` | Giữ nguyên | `Đường Liên Xã Hồng Minh – Tri Trung` | `Đường Liên Xã Hồng Minh – Tri Trung` | 12 |
| A05 | 5 | `đường tỉnh`, `ĐT`, `ĐT.`, `DT` | `ĐT.` | `Đường Tỉnh 302` | `ĐT. 302` | 892 đầy đủ + 169 alias |
| A06 | 6 | `đường huyện` | `ĐH.` | `Đường Huyện 10` | `ĐH. 10` | 141 |
| A07 | 7 | `quốc lộ`, `QL`, `QL.` | `QL.` | `Quốc Lộ 1A` | `QL. 1A` | 344 đầy đủ + 36 alias |
| A08 | 8 | `tỉnh lộ`, `TL`, `TL.` | `TL.` | `Tỉnh Lộ 206` | `TL. 206` | 40 đầy đủ + 23 alias |
| A09 | 9 | `huyện lộ`, `HL`, `HL.` | `HL.` | `Huyện Lộ 24` | `HL. 24` | 5 đầy đủ + 3 alias |
| A10 | 10 | `đại lộ` | `ĐL.` | `Đại Lộ Bốn Mùa` | `ĐL. Bốn Mùa` | 49 |
| A11 | 11 | `cao tốc` | `CT.` | `Cao tốc Liên Khương - Prenn` | `CT. Liên Khương - Prenn` | 3 |
| A12 | 12 | `xa lộ` | `XL.` | `Xa lộ Hà Nội` | `XL. Hà Nội` | 2 |
| A13 | 13 | `vành đai`, `VĐ`, `VĐ.` | `VĐ.` | `Vành Đai 5` | `VĐ. 5` | 17 đầy đủ + 3 alias |
| A14 | 14 | `hẻm`, `hèm`, `hem`, `hẽm` | Giữ `Hẻm` | `Hèm 12 Đường Số 1` | `Hẻm 12 Đường Số 1` | 28.211 chuẩn + 133 biến thể |
| A15 | 15 | `ngách` | `Ng.` | `Ngách 2 Ngõ 42` | `Ng. 2 Ngõ 42` | 5.688 |
| A16 | 16 | `ngõ` | `Ng.` | `Ngõ 62 Phố Huế` | `Ng. 62 Phố Huế` | 12.000 |
| A17 | 17 | `kiệt` | Giữ `Kiệt` | `Kiệt 83 Nguyễn Duy Hiệu` | `Kiệt 83 Nguyễn Duy Hiệu` | 1.207 |
| A18 | 18 | `đường`, `Đ.`, `đướng`, `đương`, `duong`, `dường`, `đuờng` | `Đ.` | `Dường Nguyễn Trãi` | `Đ. Nguyễn Trãi` | 11.489 riêng + 166 alias/sai dấu + 53 Unicode tổ hợp |
| A19 | 19 | `phố`, `phó` | `P.` | `Phó Nguyễn Du` | `P. Nguyễn Du` | 3.010 chuẩn + 3 biến thể |

### Điểm cần kiểm tra trong bảng rút gọn

- `Ngách` và `Ngõ` cùng viết tắt là `Ng.` theo quyết định đã duyệt.
- `Hẻm` và `Kiệt` được giữ nguyên trong mode rút gọn; biến thể sai dấu của
  `Hẻm` vẫn được chuẩn hóa về `Hẻm`.
- `Đường liên xã` và `Đường liên thôn` được giữ nguyên, không rút gọn thành
  `ĐLX.`/`ĐLT.`.
- `Đường cao tốc → CT.` và `Cao tốc → CT.` cố ý cho cùng output.
- `Đường tỉnh` và `Tỉnh lộ` là hai rule tách riêng (`ĐT.` và `TL.`).
- `Đường huyện` và `Huyện lộ` là hai rule tách riêng (`ĐH.` và `HL.`).
- Alias ASCII `DT` được chấp nhận nhưng luôn chuẩn hóa về dạng có dấu `ĐT.`.

## 4. Rule cho trường hợp xóa tiền tố

Mode xóa dùng cùng tập nhận diện với mode rút gọn nhưng hành vi được quyết định
theo từng rule: `Xóa`, `Thay thế/chuẩn hóa` hoặc `Giữ nguyên`. Tiền tố dài nhất
vẫn phải được xét trước.

| ID | Ưu tiên | Input nhận diện | Hành vi | Ví dụ input | Output |
|---|---:|---|---|---|---|
| R01 | 1 | `đường cao tốc` | Thay bằng `CT.` | `Đường cao tốc Biên Hoà - Vũng Tàu` | `CT. Biên Hoà - Vũng Tàu` |
| R02 | 2 | `đường vành đai` | Chỉ xóa `Đường` | `Đường Vành Đai 3.5` | `Vành Đai 3.5` |
| R03 | 3 | `đường liên thôn` | Giữ nguyên | `Đường Liên Thôn 2` | `Đường Liên Thôn 2` |
| R04 | 4 | `đường liên xã` | Giữ nguyên | `Đường Liên Xã Hồng Minh – Tri Trung` | `Đường Liên Xã Hồng Minh – Tri Trung` |
| R05 | 5 | `đường tỉnh`, `ĐT`, `ĐT.`, `DT` | Chuẩn hóa `ĐT.` | `ĐT. 258` | `ĐT. 258` |
| R06 | 6 | `đường huyện` | Giữ nguyên | `Đường Huyện 10` | `Đường Huyện 10` |
| R07 | 7 | `quốc lộ`, `QL`, `QL.` | Giữ nguyên; chuẩn hóa mã | `QL2` | `QL.2` |
| R08 | 8 | `tỉnh lộ`, `TL`, `TL.` | Giữ nguyên; chuẩn hóa mã | `Tỉnh Lộ 206` | `Tỉnh Lộ 206` |
| R09 | 9 | `huyện lộ`, `HL`, `HL.` | Giữ nguyên; chuẩn hóa mã | `HL.173` | `HL.173` |
| R10 | 10 | `đại lộ` | Giữ nguyên | `Đại Lộ Bốn Mùa` | `Đại Lộ Bốn Mùa` |
| R11 | 11 | `cao tốc` | Thay bằng `CT.` | `Cao tốc Liên Khương - Prenn` | `CT. Liên Khương - Prenn` |
| R12 | 12 | `xa lộ` | Xóa | `Xa lộ Hà Nội` | `Hà Nội` |
| R13 | 13 | `vành đai`, `VĐ`, `VĐ.` | Xóa | `Vành Đai 5` | `5` |
| R14 | 14 | `hẻm`, `hèm`, `hem`, `hẽm` | Giữ nguyên; sửa sai dấu nếu có | `Hẻm 12 Đường Số 1` | `Hẻm 12 Đường Số 1` |
| R15 | 15 | `ngách` | Giữ nguyên | `Ngách 2 Ngõ 42` | `Ngách 2 Ngõ 42` |
| R16 | 16 | `ngõ` | Giữ nguyên | `Ngõ 62 Phố Huế` | `Ngõ 62 Phố Huế` |
| R17 | 17 | `kiệt` | Giữ nguyên | `Kiệt 83 Nguyễn Duy Hiệu` | `Kiệt 83 Nguyễn Duy Hiệu` |
| R18 | 18 | `đường`, `Đ.`, `đướng`, `đương`, `duong`, `dường`, `đuờng` | Xóa | `Dường Nguyễn Trãi` | `Nguyễn Trãi` |
| R19 | 19 | `phố`, `phó` | Xóa | `Phó Nguyễn Du` | `Nguyễn Du` |

## 5. Danh mục tương ứng trong Rust

Sau khi duyệt, hai mode nên dùng chung một danh mục để không bị lệch rule:

```rust,ignore
enum RemoveAction {
    Remove,
    Keep,
    Replace(&'static str),
    RemoveLeadingWord,
    NormalizeCode(&'static str),
}

enum AbbreviateAction {
    Keep,
    Replace(&'static str),
}

struct RuleSpec {
    canonical: &'static str,
    aliases: &'static [&'static str],
    abbreviate: AbbreviateAction,
    remove: RemoveAction,
    allow_attached_code: bool,
}

static RULES: &[RuleSpec] = &[
    rule("đường cao tốc", &[], AbbreviateAction::Replace("CT."), RemoveAction::Replace("CT."), false),
    rule("đường vành đai", &[], AbbreviateAction::Replace("VĐ."), RemoveAction::RemoveLeadingWord, false),
    rule("đường liên thôn", &[], AbbreviateAction::Keep, RemoveAction::Keep, false),
    rule("đường liên xã", &[], AbbreviateAction::Keep, RemoveAction::Keep, false),
    rule("đường tỉnh", &["ĐT", "ĐT.", "DT"], AbbreviateAction::Replace("ĐT."), RemoveAction::NormalizeCode("ĐT."), true),
    rule("đường huyện", &[], AbbreviateAction::Replace("ĐH."), RemoveAction::Keep, false),
    rule("quốc lộ", &["QL", "QL."], AbbreviateAction::Replace("QL."), RemoveAction::Keep, true),
    rule("tỉnh lộ", &["TL", "TL."], AbbreviateAction::Replace("TL."), RemoveAction::Keep, true),
    rule("huyện lộ", &["HL", "HL."], AbbreviateAction::Replace("HL."), RemoveAction::Keep, true),
    rule("đại lộ", &[], AbbreviateAction::Replace("ĐL."), RemoveAction::Keep, false),
    rule("cao tốc", &[], AbbreviateAction::Replace("CT."), RemoveAction::Replace("CT."), false),
    rule("xa lộ", &[], AbbreviateAction::Replace("XL."), RemoveAction::Remove, false),
    rule("vành đai", &["VĐ", "VĐ."], AbbreviateAction::Replace("VĐ."), RemoveAction::Remove, true),
    rule("hẻm", &["hèm", "hem", "hẽm"], AbbreviateAction::Keep, RemoveAction::Keep, false),
    rule("ngách", &[], AbbreviateAction::Replace("Ng."), RemoveAction::Keep, false),
    rule("ngõ", &[], AbbreviateAction::Replace("Ng."), RemoveAction::Keep, false),
    rule("kiệt", &[], AbbreviateAction::Keep, RemoveAction::Keep, false),
    rule("đường", &["Đ.", "đướng", "đương", "duong", "dường", "đuờng"], AbbreviateAction::Replace("Đ."), RemoveAction::Remove, false),
    rule("phố", &["phó"], AbbreviateAction::Replace("P."), RemoveAction::Remove, false),
];
```

Hàm `rule` ở trên chỉ minh họa representation nội bộ. Rule thực tế có thể dùng
struct literal hoặc constructor `const fn`.

`AbbreviateAction::Keep` và `RemoveAction::Keep` giữ input nếu prefix đã đúng;
nếu khớp alias sai dấu thì thay alias bằng canonical prefix. Với alias mã đường,
mã vẫn được chuẩn hóa dấu chấm.

Mode được áp dụng sau khi matcher trả về rule, dạng input đã khớp và remainder:

```rust,ignore
match mode {
    Mode::Abbreviate => apply_abbreviate_action(rule, matched_form, remainder),
    Mode::Remove => apply_remove_action(rule, matched_form, remainder),
}

capitalize_first_letter(output)
```

## 6. Các token passthrough

Các token dưới đây được giữ nguyên trong cả hai mode. Chúng không được thêm vào
danh mục biến đổi prefix; output chỉ áp dụng quy tắc viết hoa chữ cái đầu chung.

Ví dụ: `cầu vượt Đường Sắt` → `Cầu vượt Đường Sắt`; mọi ký tự và khoảng trắng
khác được giữ nguyên.

| Token/cụm từ | Số lượng quan sát | Hành vi |
|---|---:|---|
| `cầu` | 4.782 | Giữ nguyên; viết hoa chữ cái đầu |
| `cầu vượt` | 314 | Giữ nguyên; viết hoa chữ cái đầu |
| `vòng xoay` | 170 | Giữ nguyên; viết hoa chữ cái đầu |
| `cống` | 121 | Giữ nguyên; viết hoa chữ cái đầu |
| `hầm` | 95 | Giữ nguyên; viết hoa chữ cái đầu |
| `xóm` | 88 | Giữ nguyên; viết hoa chữ cái đầu |
| `lối` | 87 | Giữ nguyên; viết hoa chữ cái đầu |
| `khu` | 84 | Giữ nguyên; viết hoa chữ cái đầu |
| `dãy` | 78 | Giữ nguyên; viết hoa chữ cái đầu |
| `tuyến` | 71 | Giữ nguyên; viết hoa chữ cái đầu |
| `lô` | 60 | Giữ nguyên; viết hoa chữ cái đầu |
| `đèo` | 50 | Giữ nguyên; viết hoa chữ cái đầu |
| `nhánh` | 48 | Giữ nguyên; viết hoa chữ cái đầu |
| `bến` | 46 | Giữ nguyên; viết hoa chữ cái đầu |
| `đê` | 36 | Giữ nguyên; viết hoa chữ cái đầu |
| `trạm` | 34 | Giữ nguyên; viết hoa chữ cái đầu |
| `rạch` | 32 | Giữ nguyên; viết hoa chữ cái đầu |
| `ấp` | 27 | Giữ nguyên; viết hoa chữ cái đầu |
| `chợ` | 25 | Giữ nguyên; viết hoa chữ cái đầu |
| `kênh` | 22 | Giữ nguyên; viết hoa chữ cái đầu |
| `thôn` | 22 | Giữ nguyên; viết hoa chữ cái đầu |

## 7. Biến thể sai dấu được chuẩn hóa

| Biến thể | Số lượng ở token đầu | Chuẩn hóa thành |
|---|---:|---|
| `Hèm` | 96 | `Hẻm` |
| `Hem` | 25 | `Hẻm` |
| `Hẽm` | 12 | `Hẻm` |
| `Đướng` | 58 | `Đường` |
| `Đường` | 53 | `Đường` bằng NFC; không cần alias riêng |
| `Đương` | 28 | `Đường` |
| `Duong` | 13 | `Đường` |
| `Dường` | 11 | `Đường` |
| `Đuờng` | 6 | `Đường` |
| `Phó` | 3 | `Phố` |

Chỉ sửa khi biến thể xuất hiện ở đầu chuỗi và khớp ranh giới prefix. Không thay
các từ tương tự ở giữa tên đường.

## 8. Quyết định đã chốt

1. Không rút gọn `Đường liên xã` và `Đường liên thôn`.
2. `Đường tỉnh` và `Tỉnh lộ` là hai rule tách riêng.
3. `Đường huyện` và `Huyện lộ` là hai rule tách riêng.
4. Chấp nhận alias `DT`, nhưng output luôn dùng dạng có dấu `ĐT.`.
5. Không biến đổi các token trong mục 6; chỉ viết hoa chữ cái đầu.
6. Khi không khớp, giữ nguyên mọi ký tự còn lại; không trim/co khoảng trắng.

Cả 19 rule trong hai bảng được bật trong `0.1.0`. Bảng này và fixture nghiệm thu
là nguồn review; `src/rules.rs` là danh mục được crate thực thi.
