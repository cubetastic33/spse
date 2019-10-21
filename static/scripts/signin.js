$('.formInput input').each(function() {
	if ($(this).val() !== '') {
		$(`#${this.id} + label`).animate({
			'fontSize': '0.8rem',
			'top': '-0.7rem',
			'padding': '0.25rem'
		}, 80);
	}
	$(this).focusin(() => {
		$(`#${this.id} + label`).animate({
			'fontSize': '0.8rem',
			'top': '-0.7rem',
			'padding': '0.25rem'
		}, 80);
	});
	$(this).focusout(function() {
		if ($(this).val() === '') {
			$(`#${this.id} + label`).animate({
				'fontSize': '1rem',
				'top': '1rem',
				'padding': 0
			}, 80);
		}
	});
	$(this).keyup(function () {
		$(this).parent().attr('class', 'formInput');
		$(`#${this.id} ~ .helper-text`).empty();
	});
});

$('form').submit(e => {
	e.preventDefault();
	if ($('#name').val().length === 0) {
		$('#name').parent().attr('class', 'formInput error');
		$('#name ~ .helper-text').text('Enter the name you registered with');
		showToast('Errors in form');
		return;
	}
	if ($('#password').val().length === 0) {
		$('#password').parent().attr('class', 'formInput error');
		$('#password ~ .helper-text').text('Enter your password');
		showToast('Errors in form');
		return;
	}

	showToast('Please wait...', 10000);

	$.ajax({
		type: 'POST',
		url: '/signin_user',
		data: {
			name: $('#name').val(),
			password: $('#password').val()
		},
		success: result => {
			console.log(result);
			showToast(result, 10000);
			if (result === 'success') window.location.href = '/project';
		}
	});
});
