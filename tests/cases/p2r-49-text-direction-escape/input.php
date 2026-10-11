<?php

declare(strict_types=1);

$s = "a\u{202E}b";
echo $s, "\n";
echo "x\u{2066}y\u{2069}z\u{202A}\n";
$t = "{$s}\u{202D}c";
echo $t, "\n";
