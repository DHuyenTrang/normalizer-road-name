# Phân tích và thiết kế Road Name Normalizer

## 1. Trạng thái tài liệu

Đây là tài liệu thiết kế và ghi nhận quyết định cho phiên bản `0.1.0` của thư
viện. Crate và bộ quy tắc đã được triển khai; các nhận định về dữ liệu dựa trên
snapshot 78.615 bản ghi được mô tả trong
[Hồ sơ dữ liệu](data-profile.md).

## 2. Bối cảnh và vấn đề

Nhiều hệ thống cùng nhận tên đường từ nguồn dữ liệu khác nhau. Cùng một địa danh
có thể xuất hiện dưới các dạng:

- `Đường Nguyễn Trãi`;
- `đường Nguyễn Trãi`;
- `Đ. Nguyễn Trãi`;
- `Nguyễn Trãi`.

Nếu mỗi dự án tự xử lý bằng phép thay chuỗi hoặc regex riêng, kết quả dễ không
đồng nhất, có thể thay nhầm từ ở giữa chuỗi và khó quản lý khi danh sách tiền tố
thay đổi. Một crate dùng chung sẽ gom thuật toán, quy tắc và bộ kiểm thử vào một
nơi.

## 3. Mục tiêu thiết kế

### 3.1. Mục tiêu

- Cung cấp kết quả xác định: cùng đầu vào và phiên bản luôn cho cùng đầu ra.
- Không làm hỏng tên riêng: chỉ biến đổi tiền tố đã khớp.
- Hỗ trợ dữ liệu tiếng Việt và không cắt chuỗi sai biên UTF-8.
- Cho phép tái sử dụng một normalizer trong nhiều luồng.
- Quản lý rule tập trung trong code, có review, test và version rõ ràng.
- Giữ public API nhỏ: một cụm từ, một chế độ và một kết quả.

### 3.2. Không phải mục tiêu

- Phân tích một địa chỉ hoàn chỉnh.
- Sửa lỗi chính tả hoặc nhận diện gần đúng.
- Dịch, phiên âm hay bỏ dấu.
- Tự động suy luận quy tắc từ dữ liệu.
- Thay thế mọi từ giống tiền tố ở bất kỳ vị trí nào.

## 4. Thuật ngữ

| Thuật ngữ | Ý nghĩa |
|---|---|
| Tên đường | Chuỗi đầu vào cần chuẩn hóa |
| Tiền tố | Cụm từ chỉ loại đường ở đầu chuỗi, ví dụ `Đường`, `Quốc lộ` |
| Tên riêng | Phần còn lại sau tiền tố, ví dụ `Nguyễn Trãi`, `1A` |
| Dạng viết tắt | Chuỗi thay thế tiền tố, ví dụ `Đ.`, `QL.` |
| Quy tắc | Cặp tiền tố nhận diện và dạng viết tắt, kèm độ ưu tiên |
| Chế độ | Cách xử lý tiền tố: viết tắt, loại bỏ hoặc giữ nguyên |

## 5. Yêu cầu chức năng

### FR-01 — Viết tắt tiền tố

Khi `Mode::Abbreviate`, áp dụng output hard-code của rule. Phần lớn tiền tố được
viết tắt; riêng `Hẻm` và `Kiệt` được giữ nguyên theo quyết định nghiệp vụ.

### FR-02 — Loại bỏ tiền tố

Khi `Mode::Remove`, áp dụng `RemoveAction` của rule: xóa, giữ nguyên, thay thế
hoặc chuẩn hóa. Không phải mọi loại đều bị xóa; bảng hành vi chính thức nằm tại
[Bảng rule đề xuất](rules-catalog.md).

### FR-03 — Chuẩn hóa output

Mọi output được viết hoa chữ cái đầu. Các alias sai dấu ở đầu chuỗi và mã đường
như `QL2`, `ĐT261D` được đưa về dạng canonical đã duyệt.

### FR-04 — Không khớp

Nếu không có quy tắc khớp, giữ nguyên mọi ký tự của đầu vào và chỉ viết hoa chữ
cái đầu. Không trim hoặc co khoảng trắng, không báo lỗi.

### FR-05 — Rule nằm trong code

Crate dùng một danh mục rule hard-code. Ứng dụng gọi không truyền rule hoặc regex
từ runtime. Thêm/sửa/xóa rule yêu cầu cập nhật mã nguồn, test và phát hành phiên
bản crate mới. Danh mục review nằm tại [Bảng rule đề xuất](rules-catalog.md).

### FR-06 — So khớp tiền tố an toàn

Chỉ khớp tại đầu chuỗi và phải kết thúc ở cuối chuỗi hoặc trước ranh giới khoảng
trắng. Không dùng phép `replace` toàn cục.

## 6. Yêu cầu phi chức năng

- **An toàn:** không dùng `unsafe` nếu không có lý do và benchmark chứng minh.
- **Hiệu năng:** rule là dữ liệu `const`/`static`; mỗi lần chuẩn hóa không biên
  dịch regex hay dựng cấu hình.
- **Đồng thời:** hàm không giữ mutable state và có thể gọi an toàn từ nhiều luồng.
- **Tương thích:** công bố MSRV và kiểm tra trong CI.
- **Khả năng bảo trì:** quy tắc mặc định được khai báo dạng dữ liệu và có test,
  không rải rác trong các nhánh điều kiện.

## 7. Các quyết định thiết kế

### 7.1. Ưu tiên bộ so khớp literal thay vì regex công khai

Nhu cầu cốt lõi là khớp tiền tố literal. Nhận trực tiếp regex từ người dùng làm
tăng rủi ro quy tắc quá rộng, xung đột neo đầu chuỗi và phụ thuộc vào chi tiết
engine. API phiên bản đầu nên nhận chuỗi literal và tự xử lý ranh giới.

Thư viện có thể dùng `regex` ở bên trong nếu benchmark cho thấy phù hợp, nhưng
đó là chi tiết triển khai. Với số lượng quy tắc nhỏ, duyệt danh sách đã sắp xếp
thường đơn giản và đủ nhanh. Nếu số quy tắc tăng lớn, có thể thay bằng trie mà
không đổi API công khai.

### 7.2. Không chuẩn hóa Unicode toàn chuỗi một cách ngầm định

Unicode NFC và NFD có thể biểu diễn cùng một chữ hiển thị bằng các byte khác
nhau. Việc chuẩn hóa toàn bộ đầu ra có thể làm thay đổi phần tên riêng ngoài ý
muốn. Phiên bản đầu đề xuất:

1. canonicalize tiền tố trong code và phần ứng viên chỉ phục vụ so khớp;
2. giữ nguyên byte của phần tên riêng khi tạo đầu ra;
3. ghi rõ dạng Unicode được hỗ trợ trong contract và thêm test NFC/NFD nếu chọn
   hỗ trợ cả hai.

Nếu cần normalization, dùng crate chuyên biệt như `unicode-normalization`, sau
khi đánh giá chi phí dependency và yêu cầu dữ liệu thực tế.

### 7.3. Không phân biệt hoa/thường khi so khớp

So khớp cần case-insensitive nhưng không nên biến đổi tên riêng. Case folding chỉ
áp dụng lên vùng dùng để nhận diện tiền tố. Dạng đầu ra lấy từ rule (`Đ.`,
`QL.`), do đó kết quả ổn định bất kể cách viết tiền tố đầu vào.

### 7.4. Tiền tố dài nhất thắng

Quy tắc được sắp giảm dần theo độ dài logic. Khi `quốc lộ` và `quốc` cùng tồn
tại, `quốc lộ` phải được thử trước. Nếu hai quy tắc canonicalize về cùng một
tiền tố, unit test phải báo xung đột thay vì phụ thuộc thứ tự ngầm định.

### 7.5. Không lỗi trên dữ liệu đầu vào

Chuỗi rỗng, chỉ có khoảng trắng hoặc không khớp đều là dữ liệu hợp lệ. Public API
trả `String`, không cần `Result` vì caller không cung cấp cấu hình có thể lỗi.

## 8. Kiến trúc đề xuất

```text
Rule const/static ───────────────┐
                                ▼
Input + Mode ────▶┌──────────────────────┐────▶ Output
                  │ normalize            │
                  │ match + transform    │
                  └──────────────────────┘
```

Các module:

- `rules`: `RuleSpec` nội bộ và danh sách rule hard-code theo đúng độ ưu tiên;
- `normalizer`: tìm khớp và dựng kết quả;
- `lib`: public API và rustdoc.

Không cần module I/O: thư viện không đọc file cấu hình và không truy cập mạng.

## 9. Mô hình API

```rust,ignore
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Abbreviate,
    Remove,
}

pub fn normalize(input: &str, mode: Mode) -> String;
```

### 9.1. Cân nhắc `Cow<'a, str>`

`normalize` có thể trả `Cow<'a, str>` để tránh cấp phát khi không đổi chuỗi,
nhưng việc viết hoa chữ cái đầu hoặc Unicode-normalize có thể vẫn cần cấp phát.
Đề xuất bắt đầu bằng `String`; chỉ tối ưu sang `Cow` sau khi benchmark chỉ ra cấp
phát là nút thắt. Thay đổi kiểu trả về là breaking change, nên quyết định này cần
được chốt trước `1.0.0`.

### 9.2. Mô hình rule nội bộ

```rust,ignore
struct RuleSpec {
    canonical: &'static str,
    aliases: &'static [&'static str],
    abbreviate: AbbreviateAction,
    remove: RemoveAction,
}
```

`RuleSpec` không public. Các bất biến như trường rỗng, alias trùng và thứ tự sai
được kiểm tra bằng unit test; chúng không trở thành lỗi runtime của caller.

## 10. Thuật toán xử lý

### 10.1. Chuẩn bị rule trong code

1. Khai báo canonical prefix, alias, `AbbreviateAction` và `RemoveAction` bằng
   literal.
2. Sắp danh sách theo prefix/alias dài nhất trước.
3. Unit test từ chối trường rỗng, alias trùng hoặc rule sai thứ tự.
4. Mỗi input được NFC-normalize/case-fold chỉ cho mục đích so khớp.

Không có bước build rule tại runtime.

### 10.2. Giai đoạn normalize

Pseudo-code:

```text
if input is empty: return ""

for rule in rules_by_longest_prefix:
    if input starts with rule.prefix (case-insensitive)
       and match ends at end-of-input or whitespace boundary:
        remainder := input after matched prefix, preserving original bytes
        output := apply mode and the rule-specific action
        return capitalize_first_letter(output)

return capitalize_first_letter_without_other_changes(input)
```

Với cách duyệt tuyến tính, độ phức tạp xấu nhất xấp xỉ `O(R × P)`, trong đó `P`
là độ dài tiền tố so sánh. Đây là lựa chọn hợp lý cho vài chục quy tắc. Cần
benchmark trước khi đưa vào đường xử lý hàng triệu bản ghi.

### 10.3. Ranh giới từ

Phiên bản đầu coi Unicode whitespace (`char::is_whitespace`) là ranh giới hợp
lệ. Dấu `.` không tự động là ranh giới, vì đầu vào viết tắt (`Đ. Nguyễn Trãi`)
nên được xử lý bằng một alias/rule rõ ràng nếu cần idempotency.

## 11. Idempotency và xử lý một tiền tố

`Mode::Abbreviate` bảo đảm:

```text
normalize(normalize(x, Mode::Abbreviate), Mode::Abbreviate)
    == normalize(x, Mode::Abbreviate)
```

Để đạt được với `Abbreviate`, mỗi rule hard-code nhận alias cho cả dạng đầy đủ và
dạng viết tắt. Không tự suy ra alias bằng cách bỏ dấu chấm vì có thể tạo xung
đột.

Một mô hình mở rộng có thể là:

```rust,ignore
RuleSpec {
    canonical: "đường",
    aliases: &["đ."],
    abbreviate: AbbreviateAction::Replace("Đ."),
}
```

`Mode::Remove` chỉ xử lý một tiền tố mỗi lần nên không thể bảo đảm idempotency
tuyệt đối nếu phần còn lại bắt đầu bằng một prefix khác. Ví dụ `Đường Vành Đai
3.5` lần đầu thành `Vành Đai 3.5`, và lời gọi thứ hai tiếp tục áp dụng rule
`Vành Đai`. Đây là hành vi chủ ý; caller không nên lặp `Remove` đến fixed point.

## 12. Bộ quy tắc mặc định

Danh sách dưới đây là ứng viên rút ra từ dữ liệu thực tế nhưng vẫn cần chủ sở hữu
nghiệp vụ phê duyệt. Số lượng là số bản ghi khớp **độc quyền** theo nguyên tắc
tiền tố dài nhất thắng; vì vậy `đường` không bao gồm `đường tỉnh`, `đường huyện`
và các tiền tố ghép khác.

| Tiền tố | Số bản ghi | Viết tắt đề xuất | Ghi chú |
|---|---:|---|---|
| `hẻm` | 28.211 | Giữ `Hẻm` | Biến thể sai dấu vẫn chuẩn hóa về `Hẻm` |
| `ngõ` | 12.000 | `Ng.` | Dùng chung dạng viết tắt với `ngách` |
| `đường` | 11.489 | `Đ.` | Không gồm các tiền tố `đường ...` cụ thể |
| `ngách` | 5.688 | `Ng.` | Dùng chung dạng viết tắt với `ngõ` |
| `phố` | 3.010 | `P.` | Có thể không phù hợp với mọi địa phương |
| `kiệt` | 1.207 | Giữ `Kiệt` | Không viết tắt trong mode `Abbreviate` |
| `đường tỉnh` | 892 | `ĐT.` | Dữ liệu cũng có dạng `ĐT` và `ĐT.` |
| `quốc lộ` | 344 | `QL.` | Dữ liệu cũng có dạng `QL` và `QL.` |
| `đường huyện` | 141 | `ĐH.` | Cần chốt với dạng `huyện lộ`/`HL` |
| `đường cao tốc` | 113 | `CT.` | Không gộp mù với `cao tốc` |
| `đại lộ` | 49 | `ĐL.` | Có thể trùng cách viết tắt theo domain |
| `tỉnh lộ` | 40 | `TL.` | Cần xác định quan hệ với `đường tỉnh` |

`Đường liên xã` và `Đường liên thôn` được giữ nguyên trong mode `Abbreviate`.
`Đường tỉnh`/`Tỉnh lộ` và `Đường huyện`/`Huyện lộ` là các cặp rule tách riêng.
Alias `DT` được nhận diện nhưng output luôn chuẩn hóa về dạng có dấu `ĐT.`.

Bộ mặc định là một phần của hành vi phiên bản. Thêm quy tắc mới có thể khiến một
đầu vào trước đây được giữ nguyên nay bị biến đổi; thay đổi đó phải có test và
được ghi trong changelog.

Các dạng `Hèm`, `Hem`, `Hẽm`, `Đướng`, `Đương`, `Duong`, `Dường`, `Đuờng` và
`Phó` đã được duyệt làm alias sửa sai dấu ở đầu chuỗi. Matcher không áp dụng sửa
gần đúng cho phần tên riêng hoặc cho từ xuất hiện giữa chuỗi.

## 13. Trường hợp biên

| Trường hợp | Hành vi đề xuất |
|---|---|
| Chuỗi rỗng | Trả chuỗi rỗng |
| Chỉ khoảng trắng | Giữ nguyên |
| Không có tiền tố | Giữ nguyên ngoài việc viết hoa chữ cái đầu |
| Chỉ có tiền tố | Hành vi phụ thuộc `RemoveAction` của rule |
| Nhiều khoảng trắng sau tiền tố | Chuẩn hóa thành một khoảng trắng |
| Tiền tố dính với tên riêng | Không khớp |
| Tiền tố xuất hiện giữa chuỗi | Không thay đổi |
| Khác chữ hoa/thường | Vẫn khớp |
| Dạng viết tắt đã chuẩn | Kết quả không đổi nếu có alias |
| Emoji/ký tự ngoài BMP trong tên | Giữ nguyên, không index theo byte tùy tiện |
| Unicode NFC/NFD | Hỗ trợ cả hai khi so khớp; giữ nguyên remainder gốc |

## 14. Chiến lược kiểm thử

### 14.1. Unit test

- validation của danh mục rule hard-code;
- ưu tiên prefix dài nhất;
- từng `Mode`;
- ranh giới từ;
- Unicode và case-insensitive;
- không làm thay đổi tên riêng;
- duplicate sau canonicalization.

### 14.2. Table-driven integration test

Dùng fixture có các cột:

```text
input,mode,expected,rule_set,description
```

Fixture giúp chuyên gia nghiệp vụ review mà không cần đọc thuật toán. Nếu dùng
CSV, phải quy định encoding UTF-8 và escaping rõ ràng.

### 14.3. Property test

Các thuộc tính hữu ích:

- hàm không panic với mọi chuỗi UTF-8;
- input không khớp giữ nguyên khoảng trắng và mọi ký tự ngoài chữ cái đầu;
- phần remainder không bị đổi;
- normalize lặp lại có kết quả như lần đầu;
- mọi output có chữ cái đầu được viết hoa nếu bắt đầu bằng chữ;
- các mã dính số được chuẩn hóa idempotent (`QL2` → `QL.2`).

Có thể dùng `proptest`; nên để sau bộ test ví dụ nghiệp vụ vì property test không
thay thế việc xác nhận quy tắc đúng.

### 14.4. Benchmark

Dùng `criterion` với ba nhóm: không khớp, khớp tiền tố đầu danh sách và khớp
tiền tố cuối danh sách. Benchmark cả chi phí build và throughput normalize, nhưng
không tối ưu trước khi có số liệu.

## 15. Dependency và feature

Giữ dependency tree nhỏ. Các lựa chọn cần cân nhắc:

| Nhu cầu | Lựa chọn | Khuyến nghị ban đầu |
|---|---|---|
| Regex | `regex` | Chỉ thêm nếu triển khai thực sự cần |
| Unicode normalization | `unicode-normalization` | Chỉ thêm khi chốt hỗ trợ NFC/NFD |
| Property test | `proptest` | Chỉ là dev-dependency |
| Benchmark | `criterion` | Chỉ là dev-dependency |

Không mở feature để chọn bộ rule nghiệp vụ. Phiên bản crate xác định chính xác
bộ rule được áp dụng, giúp mọi dự án cho cùng kết quả.

## 16. Tương thích và versioning

- Tuân theo Semantic Versioning.
- Thêm variant vào enum public có thể làm hỏng `match` exhaustive của downstream;
  cân nhắc `#[non_exhaustive]` cho lỗi.
- Sửa chữ viết tắt hoặc thêm default rule là thay đổi hành vi, phải ghi changelog.
- Xóa/đổi tên public item là breaking change.
- Chốt MSRV, kiểm thử MSRV và stable mới nhất trong CI.

## 17. Rủi ro và biện pháp

| Rủi ro | Tác động | Biện pháp |
|---|---|---|
| Quy tắc nghiệp vụ chưa thống nhất | Kết quả sai giữa hệ thống | Có owner duyệt fixture và default rules |
| Thay nhầm từ không phải tiền tố | Làm hỏng tên riêng | Neo đầu chuỗi, kiểm tra ranh giới từ |
| Unicode/case folding không đầy đủ | Không khớp dữ liệu hợp lệ | Chốt contract, test NFC/NFD và tiếng Việt |
| Thêm default rule làm đổi dữ liệu cũ | Regression âm thầm | SemVer, changelog và regression fixture |
| Rule hard-code quá rộng | Sai kết quả ở mọi downstream | Bảng review, fixture và phát hành version mới |
| Cấp phát trên mỗi bản ghi | Giảm throughput | Benchmark; cân nhắc `Cow` sau khi có số liệu |

## 18. Bảo mật và quyền riêng tư

Tên đường thường không nhạy cảm, nhưng địa chỉ đầy đủ có thể là dữ liệu cá nhân.
Crate không nên log đầu vào. Nếu ứng dụng cần telemetry, chỉ ghi metric tổng hợp
như số lượng khớp/không khớp; việc log dữ liệu thô thuộc trách nhiệm của ứng dụng
tích hợp.

Không nhận regex tùy ý cũng giúp giảm bề mặt cấu hình. Rust `regex` tránh
catastrophic backtracking, nhưng literal rules vẫn dễ review hơn về mặt nghiệp
vụ.

## 19. Kế hoạch triển khai

### Giai đoạn 1 — Chốt contract

- Duyệt tên crate và phạm vi.
- Chốt bộ quy tắc mặc định, viết tắt và alias.
- Chốt hành vi Unicode NFC/NFD và idempotency.
- Tạo fixture nghiệm thu từ dữ liệu thực tế đã ẩn thông tin nhạy cảm.
- Chốt vai trò của dữ liệu tham chiếu: chỉ dùng phân tích, sinh fixture hay đóng
  gói cùng crate.

### Giai đoạn 2 — MVP `0.1.0`

- Tạo crate và public API tối thiểu.
- Triển khai literal prefix matcher và validation.
- Thêm rustdoc, unit test, integration fixture.
- Thiết lập fmt, clippy, test và doc trong CI.

### Giai đoạn 3 — Ổn định

- Chạy thử với các dự án tích hợp.
- Ghi nhận trường hợp không khớp và bổ sung fixture được duyệt.
- Benchmark bằng phân phối dữ liệu thực tế.
- Công bố MSRV, license và changelog.

### Giai đoạn 4 — Mở rộng khi có nhu cầu

- API batch;
- tối ưu trie hoặc `Cow` dựa trên benchmark.

## 20. Tiêu chí nghiệm thu MVP

- API công khai có rustdoc và ví dụ compile được.
- Mọi quy tắc mặc định có ca dương tính và âm tính.
- Không thay thế tiền tố ở giữa chuỗi hoặc khi thiếu ranh giới từ.
- Kết quả `Abbreviate` idempotent cho mọi rule/alias mặc định.
- Không panic với input UTF-8 bất kỳ trong property test.
- `cargo fmt`, `cargo clippy -D warnings`, `cargo test` và `cargo doc` đều thành
  công trên MSRV và stable.
- Bộ quy tắc và fixture đã được chủ sở hữu nghiệp vụ duyệt.

## 21. Quyết định phát hành còn mở

Tên crate, 19 rule, alias, NFC/NFD, xử lý một prefix, bảo toàn whitespace và MSRV
đã được khóa cho `0.1.0`. Phạm vi phát hành và license vẫn cần được chủ sở hữu
chọn trước khi publish crate ra bên ngoài; manifest hiện đặt `publish = false`.
