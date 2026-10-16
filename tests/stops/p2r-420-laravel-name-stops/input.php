<?php

declare(strict_types=1);

namespace Illuminate\Database;

// A class of the input with the name of one of Laravel's classes of P2R-420 is no exception
// class of the input, so it stops, and so does its throw. Its catch names Laravel's class.
class MultipleRecordsFoundException extends \RuntimeException
{
}

function many(): void
{
    throw new MultipleRecordsFoundException('many');
}

function caught(): string
{
    try {
        many();
        return 'none';
    } catch (MultipleRecordsFoundException $e) {
        return $e->getMessage();
    }
}
