#include <variant>
#include <optional>
#include <any>
#include "map"
#include "string"
#include "vector"
// imports to actually bind Rust things to C/C++
#define RHashMap map
#define RString string
#define Vec vector
#define ROption optional
#define NaiveDateTime time_t
#define NaiveDate time_t
#define Result variant
#define PathBuf string
#define RStr const char *
#define Client any
using namespace std;