#include <iostream>
#include <string>
#include <vector>

#ifdef __cplusplus
extern "C" {
#endif
    typedef struct FFIToken FFIToken;
    struct FFIToken {
        char* token_type;
        const unsigned int token_type_length;
        char* literal;
        const unsigned int literal_length;
    };

    typedef struct FFITokenArray FFITokenArray;
    struct FFITokenArray {
        FFIToken* tokens;
        unsigned int length;
    };
#ifdef __cplusplus
}
#endif

typedef struct Token Token;
struct Token {
    std::string type;
    std::string literal; 
};

#ifdef __cplusplus
extern "C" {
#endif
    FFITokenArray tokenizer(char const*const tokenized, size_t length);
#ifdef __cplusplus
}
#endif

void repl(void);
Token cast_ffi_token_to_cxx_token(FFIToken);
std::vector<Token> cast_ffi_token_array_to_token_vector(FFITokenArray);

static const std::string PROMPT = ">>> ";
int main(int argc, char** argv) {
    std::cout << "Welcome to the Monkey programming language" << std::endl;
    repl();
    return EXIT_SUCCESS;
}

void repl() {
    std::string buffer;
    while (true) {
        std::getline(std::cin, buffer);
        if (buffer == "\\q") break;
        std::vector<Token> tokens = cast_ffi_token_array_to_token_vector(tokenizer(buffer.c_str(), buffer.size()));
        
    }
}
Token cast_ffi_token_to_cxx_token(FFIToken token) {
    // Convert the Rust string to a C-like String then it convert it to the std::string C++ one.
    // As it does it convert the `ffitoken.token_type` field to be correctly casted into C++ std::string
    char *c_str_tmp_token_field { (char*)malloc(token.token_type_length+1) };
    for (int i=0 ; i < token.token_type_length ; ++i)
        c_str_tmp_token_field[i] = token.token_type[i];
    c_str_tmp_token_field[token.token_type_length] = '\0';
    std::string token_type = std::string(c_str_tmp_token_field);

    // Now time to convert the ffitoken literal field
    c_str_tmp_token_field = (char*)malloc(token.literal_length+1);
    for (int i { 0 } ; i < token.literal_length ; ++i)
        c_str_tmp_token_field[i] = token.literal[i];
    c_str_tmp_token_field[token.literal_length] = '\0';
    std::string token_literal = std::string(c_str_tmp_token_field);
    // Return the normalized C++ std::string correct typed, and makes the token on
    return { .type = token_type, .literal = token_literal };
}
std::vector<Token> cast_ffi_token_array_to_token_vector(FFITokenArray tokens) {
    std::vector<Token> returned;
    for (unsigned int it_curr_token { 0 } ; it_curr_token < tokens.length ; ++it_curr_token)
        returned.push_back(cast_ffi_token_to_cxx_token(tokens.tokens[it_curr_token]));
    return returned;
}