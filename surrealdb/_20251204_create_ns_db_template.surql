DEFINE NAMESPACE IF NOT EXISTS template;
use ns template;
DEFINE DATABASE IF NOT EXISTS template;

DEFINE USER IF NOT EXISTS template_user on DATABASE PASSWORD 'template_pass';