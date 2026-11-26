<?php

declare(strict_types=1);

function say(string $s): void
{
    echo $s, "\n";
}

final class Log
{
    public function write(string $s): void
    {
        echo $s, "\n";
    }

    public static function note(string $s): void
    {
        echo $s, "\n";
    }
}

echo say('a');
$b = (new Log())->write('b');
echo $b === null ? "none\n" : "some\n";
$c = Log::note('c');
echo $c === null ? "none\n" : "some\n";
