<?php

declare(strict_types=1);

enum Level: string
{
    case Low = 'low';
    case High = 'high';
}

// new without an argument list passes each default as a call with no arguments does.
class OptionsException extends \Exception
{
    /**
     * @param list<int> $codes
     */
    public function __construct(?string $label = 'dflt', ?Level $level = Level::High, array $codes = [])
    {
        parent::__construct(($label ?? 'none') . ' ' . ($level === null ? 'null' : $level->value) . ' ' . count($codes));
    }
}

try {
    throw new OptionsException;
} catch (OptionsException $e) {
    echo $e->getMessage(), "\n";
}
try {
    throw new OptionsException();
} catch (OptionsException $e) {
    echo $e->getMessage(), "\n";
}
