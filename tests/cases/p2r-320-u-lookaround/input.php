<?php

declare(strict_types=1);

final class Names
{
    private const string MAC = '/(?<!\p{L})mac(?! )/u';

    public static function fix(string $name): string
    {
        $mc = \str_contains($name, 'mc') && \preg_match('#(?<!\p{L})mc(?! )#u', $name) === 1;
        $mac = \str_contains($name, 'mac') && \preg_match(self::MAC, $name) === 1;
        return ($mc ? 'mc' : '-') . ($mac ? 'mac' : '-');
    }
}

function forms(string $s): string
{
    $out = '';
    if (preg_match('~(?<!\p{L})ab1(?! )~u', $s)) {
        $out .= 'a';
    }
    $out .= preg_match('{(?<!\p{L})B(?! )}u', $s) !== 0 ? 'b' : '-';
    $out .= preg_match('#(?<!\p{L})aa(?! )#u', $s) === 0 ? '-' : 'c';
    $out .= !preg_match('#(?<!\p{L})7(?! )#u', $s) ? '-' : 'd';
    $out .= preg_match('#(?<!\p{L})aba(?! )#u', $s) !== 1 ? '-' : 'e';
    $out .= preg_match('#(?<!\p{L})B(?! )#uu', $s) === 1 ? 'f' : '-';
    return $out;
}

$names = [
    '', 'mc', 'mcx', 'xmc', 'mc ', ' mc', 'o\'mc', '1mc', '-mac', 'mac',
    "mc\n", "mc\t", "mc\u{00A0}", "\u{00A0}mc", "émc", "ßmac", "中mc", "дmc",
    "λmac", "e\u{0301}mc", "\u{1F600}mc", "mc\u{3000}", 'mac mac', 'xmac mac',
    'macx', 'xmcmc', 'mcmc ', 'mc mc', 'xmc mca',
];
foreach ($names as $name) {
    echo '[', $name, '] ', Names::fix($name), "\n";
}
$subjects = ['ab1', 'xab1', 'ab1 ', '-ab1-', 'B', 'aB', 'B ', 'aaa', 'baa a', 'aa ',
    '7', 'a7', '17', '7 ', 'ababa', 'xaba aba', 'aba ', 'éaba', '9aba'];
foreach ($subjects as $subject) {
    echo '[', $subject, '] ', forms($subject), "\n";
}
