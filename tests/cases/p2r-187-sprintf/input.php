<?php

declare(strict_types=1);

function line(string $name, int $n): string
{
    return sprintf('%s has %d items (100%%) {ok}', $name, $n);
}

echo line('box', 3), ' ', sprintf('%s-%s', 7, 'x'), ' ', sprintf('plain'), "\n";
echo sprintf('%d%%', -5), ' ', sprintf('é%sé', 'ü'), "\n";
