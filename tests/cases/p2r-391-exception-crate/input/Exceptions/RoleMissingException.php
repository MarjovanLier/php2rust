<?php

declare(strict_types=1);

namespace App\Exceptions;

// Its parent, which holds nothing, is declared in another file of the crate.
final class RoleMissingException extends MissingException {}
