#include <variant>
#include <optional>
#include <any>
#include "map"
#include "string"
#include "vector"
// imports to actually bind Rust things to C/C++
#define HashMap map
#define String string
#define Vec vector
#define Option optional
#define NaiveDateTime time_t
#define NaiveDate time_t
#define Result variant
#define PathBuf string
#define str const char *
#define Client any
using namespace std;