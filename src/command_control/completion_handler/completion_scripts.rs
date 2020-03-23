pub struct CompletionScript { }

    impl CompletionScript {
        pub fn bash() {
            println!("{}",r#"
    _orderitem() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            orderitem)
                cmd="orderitem"
                ;;
            
            bash)
                cmd+="__bash"
                ;;
            completions)
                cmd+="__completions"
                ;;
            configuration)
                cmd+="__configuration"
                ;;
            elvish)
                cmd+="__elvish"
                ;;
            fish)
                cmd+="__fish"
                ;;
            help)
                cmd+="__help"
                ;;
            powershell)
                cmd+="__powershell"
                ;;
            set)
                cmd+="__set"
                ;;
            zsh)
                cmd+="__zsh"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        orderitem)
            opts=" -r -d -v -h -o -c -u -s -q -n -t  --ready-to-ship --seed --daemon --dry-run --verbose --help --order-id --status --customer-id --payment-id --shipping-id --upload-id --sku --quantity --discount --remove-discount --notes --timeout --port --host  <method>  configuration help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --order-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --status)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --customer-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --payment-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --shipping-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --upload-id)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -u)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --sku)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --quantity)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -q)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --discount)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --remove-discount)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --notes)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
        orderitem__configuration)
            opts=" -h -V  --help --version   set completions help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        orderitem__configuration__completions)
            opts=" -h -V  --help --version   bash fish zsh powershell elvish help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        orderitem__configuration__completions__bash)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        orderitem__configuration__completions__elvish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        orderitem__configuration__completions__fish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        orderitem__configuration__completions__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        orderitem__configuration__completions__powershell)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        orderitem__configuration__completions__zsh)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        orderitem__configuration__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        orderitem__configuration__set)
            opts=" -h -V  --help --version  <field> <value> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        orderitem__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _orderitem -o bashdefault -o default orderitem

    "#);
        }

        pub fn fish() {
            println!("{}",r#"
    complete -c orderitem -n "__fish_use_subcommand" -s o -l order-id -d 'Order ID'
complete -c orderitem -n "__fish_use_subcommand" -l status -d 'Order status'
complete -c orderitem -n "__fish_use_subcommand" -s c -l customer-id -d 'Customer ID'
complete -c orderitem -n "__fish_use_subcommand" -l payment-id -d 'Payment ID'
complete -c orderitem -n "__fish_use_subcommand" -l shipping-id -d 'Shipping ID'
complete -c orderitem -n "__fish_use_subcommand" -s u -l upload-id -d 'Upload ID'
complete -c orderitem -n "__fish_use_subcommand" -s s -l sku -d 'Sku Model ID'
complete -c orderitem -n "__fish_use_subcommand" -s q -l quantity -d 'How many items'
complete -c orderitem -n "__fish_use_subcommand" -l discount -d 'Apply discount'
complete -c orderitem -n "__fish_use_subcommand" -l remove-discount -d 'Remove discount'
complete -c orderitem -n "__fish_use_subcommand" -s n -l notes -d 'Add/Append a note for order'
complete -c orderitem -n "__fish_use_subcommand" -s t -l timeout -d 'Request time to live in milliseconds'
complete -c orderitem -n "__fish_use_subcommand" -l port -d 'Daemon mode port'
complete -c orderitem -n "__fish_use_subcommand" -l host -d 'Daemon mode host'
complete -c orderitem -n "__fish_use_subcommand" -s r -l ready-to-ship -d 'Mark ready to ship order'
complete -c orderitem -n "__fish_use_subcommand" -l seed -d 'Seed database on initialization if it doesn\'t exist'
complete -c orderitem -n "__fish_use_subcommand" -s d -l daemon -d 'Daemon mode'
complete -c orderitem -n "__fish_use_subcommand" -l dry-run -d 'Don\'t run commands only log'
complete -c orderitem -n "__fish_use_subcommand" -s v -l verbose -d 'Enable verbose logging'
complete -c orderitem -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_use_subcommand" -f -a "configuration" -d 'Configuration options'
complete -c orderitem -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c orderitem -n "__fish_seen_subcommand_from configuration" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_seen_subcommand_from configuration" -s V -l version -d 'Prints version information'
complete -c orderitem -n "__fish_seen_subcommand_from configuration" -f -a "set" -d 'Set configuration file value to something new'
complete -c orderitem -n "__fish_seen_subcommand_from configuration" -f -a "completions" -d 'Completion scripts for various shells'
complete -c orderitem -n "__fish_seen_subcommand_from configuration" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c orderitem -n "__fish_seen_subcommand_from set" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_seen_subcommand_from set" -s V -l version -d 'Prints version information'
complete -c orderitem -n "__fish_seen_subcommand_from completions" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_seen_subcommand_from completions" -s V -l version -d 'Prints version information'
complete -c orderitem -n "__fish_seen_subcommand_from completions" -f -a "bash" -d 'Bash completion script'
complete -c orderitem -n "__fish_seen_subcommand_from completions" -f -a "fish" -d 'Fish completion script'
complete -c orderitem -n "__fish_seen_subcommand_from completions" -f -a "zsh" -d 'Zsh completion script'
complete -c orderitem -n "__fish_seen_subcommand_from completions" -f -a "powershell" -d 'PowerShell completion script'
complete -c orderitem -n "__fish_seen_subcommand_from completions" -f -a "elvish" -d 'Elvish completion script'
complete -c orderitem -n "__fish_seen_subcommand_from completions" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c orderitem -n "__fish_seen_subcommand_from bash" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_seen_subcommand_from bash" -s V -l version -d 'Prints version information'
complete -c orderitem -n "__fish_seen_subcommand_from fish" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_seen_subcommand_from fish" -s V -l version -d 'Prints version information'
complete -c orderitem -n "__fish_seen_subcommand_from zsh" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_seen_subcommand_from zsh" -s V -l version -d 'Prints version information'
complete -c orderitem -n "__fish_seen_subcommand_from powershell" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_seen_subcommand_from powershell" -s V -l version -d 'Prints version information'
complete -c orderitem -n "__fish_seen_subcommand_from elvish" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_seen_subcommand_from elvish" -s V -l version -d 'Prints version information'
complete -c orderitem -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
complete -c orderitem -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
complete -c orderitem -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c orderitem -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'

    "#);
        }

        pub fn zsh() {
            println!("{}",r#"
    #compdef orderitem

autoload -U is-at-least

_orderitem() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-o+[Order ID]' \
'--order-id=[Order ID]' \
'--status=[Order status]' \
'-c+[Customer ID]' \
'--customer-id=[Customer ID]' \
'--payment-id=[Payment ID]' \
'--shipping-id=[Shipping ID]' \
'-u+[Upload ID]' \
'--upload-id=[Upload ID]' \
'-s+[Sku Model ID]' \
'--sku=[Sku Model ID]' \
'-q+[How many items]' \
'--quantity=[How many items]' \
'--discount=[Apply discount]' \
'--remove-discount=[Remove discount]' \
'-n+[Add/Append a note for order]' \
'--notes=[Add/Append a note for order]' \
'-t+[Request time to live in milliseconds]' \
'--timeout=[Request time to live in milliseconds]' \
'--port=[Daemon mode port]' \
'--host=[Daemon mode host]' \
'-r[Mark ready to ship order]' \
'--ready-to-ship[Mark ready to ship order]' \
'--seed[Seed database on initialization if it doesn'\''t exist]' \
'-d[Daemon mode]' \
'--daemon[Daemon mode]' \
'--dry-run[Don'\''t run commands only log]' \
'-v[Enable verbose logging]' \
'--verbose[Enable verbose logging]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'::method -- The type of method being requested for signing url:_files' \
":: :_orderitem_commands" \
"*::: :->orderitem" \
&& ret=0
    case $state in
    (orderitem)
        words=($line[2] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:orderitem-command-$line[2]:"
        case $line[2] in
            (configuration)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_orderitem__configuration_commands" \
"*::: :->configuration" \
&& ret=0
case $state in
    (configuration)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:orderitem-configuration-command-$line[1]:"
        case $line[1] in
            (set)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::field -- Field being targeted for change:_files' \
'::value -- Target field new value:_files' \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_orderitem__configuration__completions_commands" \
"*::: :->completions" \
&& ret=0
case $state in
    (completions)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:orderitem-configuration-completions-command-$line[1]:"
        case $line[1] in
            (bash)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Bash completion script:_files' \
&& ret=0
;;
(fish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Fish completion script:_files' \
&& ret=0
;;
(zsh)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Zsh completion script:_files' \
&& ret=0
;;
(powershell)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- PowerShell completion script:_files' \
&& ret=0
;;
(elvish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Elvish completion script:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_orderitem_commands] )) ||
_orderitem_commands() {
    local commands; commands=(
        "configuration:Configuration options" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'orderitem commands' commands "$@"
}
(( $+functions[_orderitem__configuration__completions__bash_commands] )) ||
_orderitem__configuration__completions__bash_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'orderitem configuration completions bash commands' commands "$@"
}
(( $+functions[_orderitem__configuration__completions_commands] )) ||
_orderitem__configuration__completions_commands() {
    local commands; commands=(
        "bash:Bash completion script" \
"fish:Fish completion script" \
"zsh:Zsh completion script" \
"powershell:PowerShell completion script" \
"elvish:Elvish completion script" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'orderitem configuration completions commands' commands "$@"
}
(( $+functions[_orderitem__configuration_commands] )) ||
_orderitem__configuration_commands() {
    local commands; commands=(
        "set:Set configuration file value to something new" \
"completions:Completion scripts for various shells" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'orderitem configuration commands' commands "$@"
}
(( $+functions[_orderitem__configuration__completions__elvish_commands] )) ||
_orderitem__configuration__completions__elvish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'orderitem configuration completions elvish commands' commands "$@"
}
(( $+functions[_orderitem__configuration__completions__fish_commands] )) ||
_orderitem__configuration__completions__fish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'orderitem configuration completions fish commands' commands "$@"
}
(( $+functions[_orderitem__configuration__completions__help_commands] )) ||
_orderitem__configuration__completions__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'orderitem configuration completions help commands' commands "$@"
}
(( $+functions[_orderitem__configuration__help_commands] )) ||
_orderitem__configuration__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'orderitem configuration help commands' commands "$@"
}
(( $+functions[_orderitem__help_commands] )) ||
_orderitem__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'orderitem help commands' commands "$@"
}
(( $+functions[_orderitem__configuration__completions__powershell_commands] )) ||
_orderitem__configuration__completions__powershell_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'orderitem configuration completions powershell commands' commands "$@"
}
(( $+functions[_orderitem__configuration__set_commands] )) ||
_orderitem__configuration__set_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'orderitem configuration set commands' commands "$@"
}
(( $+functions[_orderitem__configuration__completions__zsh_commands] )) ||
_orderitem__configuration__completions__zsh_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'orderitem configuration completions zsh commands' commands "$@"
}

_orderitem "$@"
    "#);
        }

        pub fn powershell() {
            println!("{}",r#"
    
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'orderitem' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'orderitem'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'orderitem' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Order ID')
            [CompletionResult]::new('--order-id', 'order-id', [CompletionResultType]::ParameterName, 'Order ID')
            [CompletionResult]::new('--status', 'status', [CompletionResultType]::ParameterName, 'Order status')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Customer ID')
            [CompletionResult]::new('--customer-id', 'customer-id', [CompletionResultType]::ParameterName, 'Customer ID')
            [CompletionResult]::new('--payment-id', 'payment-id', [CompletionResultType]::ParameterName, 'Payment ID')
            [CompletionResult]::new('--shipping-id', 'shipping-id', [CompletionResultType]::ParameterName, 'Shipping ID')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'Upload ID')
            [CompletionResult]::new('--upload-id', 'upload-id', [CompletionResultType]::ParameterName, 'Upload ID')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Sku Model ID')
            [CompletionResult]::new('--sku', 'sku', [CompletionResultType]::ParameterName, 'Sku Model ID')
            [CompletionResult]::new('-q', 'q', [CompletionResultType]::ParameterName, 'How many items')
            [CompletionResult]::new('--quantity', 'quantity', [CompletionResultType]::ParameterName, 'How many items')
            [CompletionResult]::new('--discount', 'discount', [CompletionResultType]::ParameterName, 'Apply discount')
            [CompletionResult]::new('--remove-discount', 'remove-discount', [CompletionResultType]::ParameterName, 'Remove discount')
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'Add/Append a note for order')
            [CompletionResult]::new('--notes', 'notes', [CompletionResultType]::ParameterName, 'Add/Append a note for order')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Request time to live in milliseconds')
            [CompletionResult]::new('--timeout', 'timeout', [CompletionResultType]::ParameterName, 'Request time to live in milliseconds')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'Daemon mode port')
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'Daemon mode host')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Mark ready to ship order')
            [CompletionResult]::new('--ready-to-ship', 'ready-to-ship', [CompletionResultType]::ParameterName, 'Mark ready to ship order')
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'Seed database on initialization if it doesn''t exist')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Daemon mode')
            [CompletionResult]::new('--daemon', 'daemon', [CompletionResultType]::ParameterName, 'Daemon mode')
            [CompletionResult]::new('--dry-run', 'dry-run', [CompletionResultType]::ParameterName, 'Don''t run commands only log')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Enable verbose logging')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Enable verbose logging')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('configuration', 'configuration', [CompletionResultType]::ParameterValue, 'Configuration options')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'orderitem;configuration' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set configuration file value to something new')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Completion scripts for various shells')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'orderitem;configuration;set' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'orderitem;configuration;completions' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('bash', 'bash', [CompletionResultType]::ParameterValue, 'Bash completion script')
            [CompletionResult]::new('fish', 'fish', [CompletionResultType]::ParameterValue, 'Fish completion script')
            [CompletionResult]::new('zsh', 'zsh', [CompletionResultType]::ParameterValue, 'Zsh completion script')
            [CompletionResult]::new('powershell', 'powershell', [CompletionResultType]::ParameterValue, 'PowerShell completion script')
            [CompletionResult]::new('elvish', 'elvish', [CompletionResultType]::ParameterValue, 'Elvish completion script')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'orderitem;configuration;completions;bash' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'orderitem;configuration;completions;fish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'orderitem;configuration;completions;zsh' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'orderitem;configuration;completions;powershell' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'orderitem;configuration;completions;elvish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'orderitem;configuration;completions;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'orderitem;configuration;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'orderitem;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

    "#);
        }

        pub fn elvish() {
            println!("{}",r#"
            
edit:completion:arg-completer[orderitem] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'orderitem'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'orderitem'= {
            cand -o 'Order ID'
            cand --order-id 'Order ID'
            cand --status 'Order status'
            cand -c 'Customer ID'
            cand --customer-id 'Customer ID'
            cand --payment-id 'Payment ID'
            cand --shipping-id 'Shipping ID'
            cand -u 'Upload ID'
            cand --upload-id 'Upload ID'
            cand -s 'Sku Model ID'
            cand --sku 'Sku Model ID'
            cand -q 'How many items'
            cand --quantity 'How many items'
            cand --discount 'Apply discount'
            cand --remove-discount 'Remove discount'
            cand -n 'Add/Append a note for order'
            cand --notes 'Add/Append a note for order'
            cand -t 'Request time to live in milliseconds'
            cand --timeout 'Request time to live in milliseconds'
            cand --port 'Daemon mode port'
            cand --host 'Daemon mode host'
            cand -r 'Mark ready to ship order'
            cand --ready-to-ship 'Mark ready to ship order'
            cand --seed 'Seed database on initialization if it doesn''t exist'
            cand -d 'Daemon mode'
            cand --daemon 'Daemon mode'
            cand --dry-run 'Don''t run commands only log'
            cand -v 'Enable verbose logging'
            cand --verbose 'Enable verbose logging'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand configuration 'Configuration options'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'orderitem;configuration'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand set 'Set configuration file value to something new'
            cand completions 'Completion scripts for various shells'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'orderitem;configuration;set'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'orderitem;configuration;completions'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand bash 'Bash completion script'
            cand fish 'Fish completion script'
            cand zsh 'Zsh completion script'
            cand powershell 'PowerShell completion script'
            cand elvish 'Elvish completion script'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'orderitem;configuration;completions;bash'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'orderitem;configuration;completions;fish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'orderitem;configuration;completions;zsh'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'orderitem;configuration;completions;powershell'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'orderitem;configuration;completions;elvish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'orderitem;configuration;completions;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'orderitem;configuration;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'orderitem;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}

    "#);
        }
    }
    