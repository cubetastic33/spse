$('.formInput input:not([type=checkbox]), .formInput select').each(function() {
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
	$(this).focusout(function () {
		if ($(this).val() === '') {
			$(`#${this.id} + label`).animate({
				'fontSize': '1rem',
				'top': '1rem',
				'padding': 0
			}, 80);
		}
	});
	$(this).keyup(function () {
		if (this.id === 'password' && $(this).val().length > 0 && $(this).val().length < 8) {
			$('#password').parent().attr('class', 'formInput error');
			$('#password ~ .helper-text').text('This has to be at least 8 characters');
		} else {
			$(this).parent().attr('class', 'formInput');
			if (this.id !== 'class') {
				$(`#${this.id} ~ .helper-text`).empty();
			}
		}
	});
});

$('form').submit(e => {
	e.preventDefault();
	if ($('#name').val().length === 0) {
		$('#name').parent().attr('class', 'formInput error');
		$('#name ~ .helper-text').text('Enter your name');
		showToast('Errors in form');
		return;
	}
	if ($('#email').val().length === 0 || $('#email').is(':invalid')) {
		$('#email').parent().attr('class', 'formInput error');
		$('#email ~ .helper-text').text('Enter a valid email address');
		showToast('Errors in form');
		return;
	}
	if ($('#password').val().length < 8) {
		$('#password').parent().attr('class', 'formInput error');
		$('#password ~ .helper-text').text('This has to be at least 8 characters');
		showToast('Errors in form');
		return;
	}
	if ($('#section').val().length === 0) {
		$('#section').parent().attr('class', 'formInput error');
		$('#section ~ .helper-text').text('Enter your section');
		showToast('Errors in form');
		return;
	}
	if ($('#projectID').val().length === 0) {
		$('#projectID').parent().attr('class', 'formInput error');
		$('#projectID ~ .helper-text').text('Enter the project ID');
		showToast('Errors in form');
		return;
	}
	if (!($('#guidelinesConsent').prop('checked') && $('#rulesConsent').prop('checked'))) {
		showToast("You can't participate if you don't agree with the guidelines and rules. If you still want to try, contact the administrators.");
		return;
	}
	showToast('Please wait...', 10000);
	$.ajax({
		type: 'POST',
		url: '/jointeam',
		data: {
			name: $('#name').val(),
			email: $('#email').val(),
			phone: $('#phone').val().length === 0 ? undefined : $('#phone').val(),
			password: $('#password').val(),
			grade: $('#class').val(),
			section: $('#section').val(),
			project_id: $('#projectID').val()
		},
		success: result => {
			console.log(result);
			showToast(result, 10000);
			if (result === 'Successfully sent request') window.location.href = '/project';
		}
	});
});
