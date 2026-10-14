<?php

declare(strict_types=1);

final class Tag
{
    public function __construct(public string $name) {}

    public function shown(): string
    {
        return ucfirst(strtolower($this->name));
    }
}

echo strtolower('ABC é Z'), ' ', strtoupper('abc é z'), ' ', strtoupper(''), "\n";
echo ucfirst('abc'), ' ', ucfirst('Abc'), ' ', ucfirst('éa'), ' ', ucfirst('1a'), "\n";
echo lcfirst('ABC'), ' ', lcfirst(''), ' ', lcfirst('ÉA'), "\n";
echo (new Tag('hELLO'))->shown(), "\n";
