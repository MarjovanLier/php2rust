<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\ModelNotFoundException;

// A catch alone puts Laravel's classes in the class tree; a RuntimeException is no
// ModelNotFoundException, so the second clause catches it.
function check(int $n): string
{
    try {
        if ($n > 1) {
            throw new \RuntimeException('too big');
        }
        return 'fine';
    } catch (ModelNotFoundException $e) {
        return 'missing';
    } catch (\RuntimeException $e) {
        return 'runtime: ' . $e->getMessage();
    }
}

echo check(1), ' ', check(2), "\n";
