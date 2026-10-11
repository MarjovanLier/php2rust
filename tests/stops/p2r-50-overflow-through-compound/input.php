<?php

declare(strict_types=1);

// rustc 1.99 accepts `top += 1` here, because top is mut. php2rust still refuses it, so
// that its check is never weaker than rustc's.
$top = 9223372036854775807;
$top += 1;
echo $top, "\n";
