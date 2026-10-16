<?php

declare(strict_types=1);

namespace App\Services;

use App\Exceptions\MissingException;
use App\Exceptions\RecordMissingException;
use App\Exceptions\RoleMissingException;

// This module names the function of another module, so it holds its use line (P2R-392).
function find(int $id): string
{
    if ($id < 1) {
        throw new RecordMissingException('Record', $id);
    }
    if ($id === 7) {
        throw new RoleMissingException('no role 7');
    }
    return 'record ' . $id;
}

function describe(int $id): string
{
    try {
        return find($id);
    } catch (MissingException $e) {
        return 'missing: ' . $e->getMessage();
    } catch (RecordMissingException $e) {
        return 'record missing: ' . $e->getMessage() . ' (' . $e->getCode() . ')';
    }
}
