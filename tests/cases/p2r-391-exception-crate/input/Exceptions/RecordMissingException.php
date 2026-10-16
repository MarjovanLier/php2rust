<?php

declare(strict_types=1);

namespace App\Exceptions;

// The constructor's function goes to the module of its namespace (P2R-391).
class RecordMissingException extends \RuntimeException
{
    public function __construct(string $entity, int $id)
    {
        parent::__construct(sprintf('%s %d not found', $entity, $id), 404);
    }
}
