#ifndef CLI_CORE_H
#define CLI_CORE_H

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Logging functions
void cli_log_info(const char* message);
void cli_log_warn(const char* message);
void cli_log_error(const char* message);
void cli_log_success(const char* message);

// Template functions
char* cli_get_template(const char* key);
void cli_free_string(char* ptr);

// Config functions
bool cli_load_config(const char* path);

// Progress bar functions
size_t cli_create_progress_bar(uint64_t total);
bool cli_update_progress(size_t id, uint64_t current, const char* message);
bool cli_finish_progress(size_t id, const char* message);

// Command line argument parser
typedef struct ArgParser ArgParser;

ArgParser* cli_create_arg_parser(const char* program_name);
void cli_set_parser_description(ArgParser* parser, const char* description);
bool cli_parse_args(ArgParser* parser, int argc, const char* argv[]);
char* cli_arg_parser_get(const ArgParser* parser, const char* key);
bool cli_arg_parser_has_flag(const ArgParser* parser, const char* flag);
void cli_arg_parser_print_help(const ArgParser* parser);
void cli_arg_parser_free(ArgParser* parser);

// Interactive functions
char* cli_prompt(const char* message);
bool cli_confirm(const char* message, bool default_value);
int cli_select_option(const char* message, const char* options[], size_t options_count);
char* cli_read_password(const char* prompt_message);

#ifdef __cplusplus
}
#endif

#endif